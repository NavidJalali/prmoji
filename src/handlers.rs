use crate::auth::{verify_github_signature, verify_slack_signature};
use crate::clock::Clock;
use crate::persistence::pr_repository::PrRepository;
use crate::slack::models::{self as slack_models, Emoji};
use crate::slack::SlackClient;
use crate::url_extractor::extract_pr_urls;
use crate::AppState;
use crate::{github, models::*};
use axum::body::Bytes;
use axum::extract::State;
use axum::http::{status, HeaderMap, Response};
use axum::response::IntoResponse;
use axum::Json;
use tracing::{error, info};

#[derive(serde::Serialize, Clone, Copy)]
pub struct ApiError {
  pub message: &'static str,
  pub status_code: u16,
}

impl ApiError {
  pub fn new(message: &'static str, status_code: u16) -> Self {
    Self {
      message,
      status_code,
    }
  }
}

impl IntoResponse for ApiError {
  fn into_response(self) -> axum::response::Response {
    let status_code = status::StatusCode::from_u16(self.status_code).unwrap();
    let body = serde_json::to_string(&self).unwrap();
    Response::builder()
      .status(status_code)
      .header("Content-Type", "application/json")
      .body(body.into())
      .unwrap()
  }
}

pub async fn list<S: AppState>(state: State<S>) -> Json<Vec<PR>> {
  let repo = state.pr_repository();
  let prs = repo.list().await;
  Json(prs)
}

pub async fn debug<S: AppState>(headers: HeaderMap, Json(payload): Json<serde_json::Value>) {
  let headers = headers
    .iter()
    .map(|(key, value)| (key.to_string(), value.to_str().unwrap().to_string()))
    .collect::<Vec<(String, String)>>();
  info!("Received headers: {:?}", headers);
  info!("Received payload: {}", payload.to_string());
}

pub async fn handle_github_event<S: AppState>(
  state: State<S>,
  headers: HeaderMap,
  payload: Bytes,
) -> Result<(), ApiError> {
  info!("Received headers: {:?}", headers);
  info!("Received payload: {:?}", payload);

  let x_hub_signature = headers
    .get("x-hub-signature-256")
    .ok_or(ApiError::new("Missing X-Hub-Signature-256 header", 401))?
    .to_str()
    .map_err(|_| ApiError::new("Invalid X-Hub-Signature-256 header", 400))?
    .strip_prefix("sha256=")
    .ok_or(ApiError::new("Invalid X-Hub-Signature-256 header", 400))?;

  let x_hub_signature = hex::decode(x_hub_signature).map_err(|err| {
    error!("Failed to decode X-Hub-Signature-256 header: {:?}", err);
    ApiError::new("Failed to decode X-Hub-Signature-256 header", 400)
  })?;

  let signature = verify_github_signature(
    &state.config().github.secret(),
    payload.to_vec(),
    x_hub_signature,
  );

  if !signature {
    error!("Signature mismatch");
    return Err(ApiError::new("Invalid signature", 401));
  }

  let payload = serde_json::from_slice(&payload).map_err(|e| {
    error!("Failed to parse github payload: {:?}", e);
    ApiError::new("Failed to parse github payload", 400)
  })?;

  let x_github_event = headers
    .get("X-GitHub-Event")
    .ok_or(ApiError::new("Missing X-GitHub-Event header", 400))?
    .to_str()
    .map(|raw| github::EventTypeHeader::from_raw(raw))
    .map_err(|_| ApiError::new("Invalid X-GitHub-Event header", 400))?;

  // If None we are not interested in this event
  let x_github_event = match x_github_event {
    Some(event) => event,
    None => return Ok(()),
  };

  let github_event = github::GitHubEvent::from_raw(x_github_event, payload);

  // If None we are not interested in this event
  let github::GitHubEvent { pr_url, event_type } = match github_event {
    Some(event) => event,
    None => return Ok(()),
  };

  info!("Received {:?} for {:?}", event_type, pr_url);

  let emoji = match event_type {
    github::GitHubEventType::Closed => Emoji::Deleted,
    github::GitHubEventType::Merged => Emoji::Merged,
    github::GitHubEventType::Commented { commenter: _ } => Emoji::Comment,
    github::GitHubEventType::ChangesRequested { reviewer: _ } => Emoji::ChangeRequest,
    github::GitHubEventType::Approved { approver: _ } => Emoji::Approved,
  };

  let repo = state.pr_repository();

  let prs = repo.get_by_url(pr_url).await;

  let slack = state.slack_client();

  let reactions = prs.into_iter().map(|pr| {
    slack.add_reaction(slack_models::AddReactionRequest {
      channel: pr.channel,
      name: emoji.clone(),
      timestamp: pr.timestamp,
    })
  });

  let results = futures::future::join_all(reactions).await;

  for result in results {
    match result {
      Ok(_) => info!("Successfully added reaction"),
      Err(err) => info!("Failed to add reaction: {:?}", err),
    }
  }

  Ok(())
}

pub async fn handle_slack_event<S: AppState>(
  state: State<S>,
  headers: HeaderMap,
  payload: Bytes,
) -> Result<Json<slack_models::Response>, ApiError> {
  let x_slack_signature = headers
    .get("x-slack-signature")
    .ok_or(ApiError::new("Missing X-Slack-Signature header", 401))?
    .to_str()
    .map_err(|_| ApiError::new("Invalid X-Slack-Signature header", 400))?
    .strip_prefix("v0=")
    .ok_or(ApiError::new("Invalid X-Slack-Signature header", 400))?;

  let x_slack_signature = hex::decode(x_slack_signature).map_err(|_| {
    error!("Failed to decode X-Slack-Signature header");
    ApiError::new("Failed to decode X-Slack-Signature header", 400)
  })?;

  let x_slack_request_timestamp: i64 = headers
    .get("x-slack-request-timestamp")
    .ok_or(ApiError::new(
      "Missing X-Slack-Request-Timestamp header",
      401,
    ))?
    .to_str()
    .map_err(|_| ApiError::new("Invalid X-Slack-Request-Timestamp header", 400))?
    .parse()
    .map_err(|_| ApiError::new("Invalid X-Slack-Request-Timestamp header", 400))?;

  let now = state.clock().now().timestamp();

  if now - x_slack_request_timestamp > 60 * 5 {
    error!("X-Slack-Request-Timestamp header is too old");
    return Err(ApiError::new(
      "X-Slack-Request-Timestamp header is too old",
      401,
    ));
  }

  let signature = verify_slack_signature(
    &state.config().slack.signing_secret(),
    x_slack_request_timestamp,
    payload.to_vec(),
    x_slack_signature,
  );

  if !signature {
    error!("Signature mismatch");
    return Err(ApiError::new("Invalid signature", 401));
  }

  let payload = serde_json::from_slice::<slack_models::WebookCallback>(&payload).map_err(|e| {
    error!("Failed to parse slack payload: {:?}", e);
    ApiError::new("Failed to parse slack payload", 400)
  })?;

  match payload {
    slack_models::WebookCallback::UrlVerification { challenge, .. } => {
      Ok(Json(slack_models::Response::ChallengeReply { challenge }))
    }
    slack_models::WebookCallback::EventCallback { event, .. } => {
      match event {
        slack_models::Event::Create(message) => {
          let to_insert = ToInsert::new(
            extract_pr_urls(&message.text.0),
            message.channel,
            message.event_ts,
            state.clock().now(),
          );

          let repo = state.pr_repository();

          info!("Extracted to_insert: {:?}", to_insert);
          repo.insert_all(to_insert).await
        }

        slack_models::Event::Update(update) => match update {
          slack_models::MessageUpdate::MessageChanged {
            message,
            previous_message,
            channel,
            channel_type: _,
            event_ts,
          } => {
            info!("Received message update: {:?}", message);

            let clock = state.clock();

            let to_delete = ToDelete::new(
              extract_pr_urls(&previous_message.text.0),
              channel.clone(),
              event_ts.clone(),
            );

            let to_insert = ToInsert::new(
              extract_pr_urls(&message.text.0),
              channel,
              event_ts,
              clock.now(),
            );

            info!("Extracted to_delete: {:?}", to_delete);
            info!("Extracted to_insert: {:?}", to_insert);

            let repo = state.pr_repository();

            repo.update(to_insert, to_delete).await
          }

          slack_models::MessageUpdate::MessageDeleted {
            channel,
            channel_type: _,
            event_ts,
            previous_message,
          } => {
            let to_delete =
              ToDelete::new(extract_pr_urls(&previous_message.text.0), channel, event_ts);

            info!("Extracted to_delete: {:?}", to_delete);
            let repo = state.pr_repository();
            repo.delete_all(to_delete).await
          }
        },
      }
      Ok(Json(slack_models::Response::Ok))
    }
  }
}
