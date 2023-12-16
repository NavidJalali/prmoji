use axum::{extract::State, Json};
use hyper::HeaderMap;
use tracing::{info, warn};

use crate::{
  app_state::AppState,
  clock::Clock,
  github,
  models::{ToDelete, ToInsert},
  persistence::pr_repository::PrRepository,
  slack::{
    self,
    models::{AddReactionRequest, Emoji},
    SlackClient,
  },
  url_extractor::extract_pr_urls,
};

use self::models::ApiError;

pub mod auth;
mod models;

pub async fn handle_github_webhook<S: AppState>(
  state: State<S>,
  headers: HeaderMap,
  Json(payload): Json<github::RawGitHubEvent>,
) -> Result<(), ApiError> {
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
    slack.add_reaction(AddReactionRequest {
      channel: pr.channel,
      name: emoji.clone(),
      timestamp: pr.timestamp,
    })
  });

  let results = futures::future::join_all(reactions).await;

  for result in results {
    match result {
      Ok(_) => info!("Successfully added reaction"),
      Err(err) => warn!("Failed to add reaction: {:?}", err),
    }
  }

  Ok(())
}

pub async fn handle_slack_webhook<S: AppState>(
  state: State<S>,
  Json(payload): Json<slack::models::WebookCallback>,
) -> Result<Json<slack::models::Response>, ApiError> {
  match payload {
    slack::models::WebookCallback::UrlVerification { challenge, .. } => {
      Ok(Json(slack::models::Response::ChallengeReply { challenge }))
    }
    slack::models::WebookCallback::EventCallback { event, .. } => {
      match event {
        slack::models::Event::Create(message) => {
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

        slack::models::Event::Update(update) => match update {
          slack::models::MessageUpdate::MessageChanged {
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

          slack::models::MessageUpdate::MessageDeleted {
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
      Ok(Json(slack::models::Response::Ok))
    }
  }
}
