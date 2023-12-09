use crate::database::Database;
use crate::models::*;
use crate::slack::models as slack_models;
use crate::AppState;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;
use tracing::info;

pub async fn list<S: AppState>(state: State<S>) -> Json<Vec<PR>> {
  let db = state.db();
  let prs = db
    .transactionally(|mut connection| async move { db.list(&mut connection).await })
    .await;
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

pub async fn event<S: AppState>(
  state: State<S>,
  Json(payload): Json<slack_models::WebookCallback>,
) -> Json<slack_models::Response> {
  info!("Received payload: {:?}", payload);
  match payload {
    slack_models::WebookCallback::UrlVerification { challenge, .. } => {
      Json(slack_models::Response::ChallengeReply { challenge })
    }
    slack_models::WebookCallback::EventCallback { event, .. } => {
      match event {
        slack_models::Event::Create(message) => {
          let prs = PR::from_message(&message.text.0, &message.channel.0, state.clock());
          let db = state.db();

          if !prs.is_empty() {
            info!("Extracted prs: {:?}", prs);
            db.transactionally(
              |mut connection| async move { db.insert_all(prs, &mut connection).await },
            )
            .await
          }
        }
        slack_models::Event::Update(update) => match update {
          slack_models::MessageUpdate::MessageChanged {
            message,
            previous_message,
            channel,
            channel_type: _,
            event_ts: _,
          } => {
            info!("Received message update: {:?}", message);

            let to_delete = ToDelete::from_message(&previous_message.text.0, &channel.0);
            let prs = PR::from_message(&message.text.0, &channel.0, state.clock());

            if !to_delete.is_empty() && !prs.is_empty() {
              info!("Extracted to_delete: {:?}", to_delete);
              info!("Extracted prs: {:?}", prs);

              let db = state.db();

              state
                .db()
                .transactionally(|mut connection| async move {
                  if !to_delete.is_empty() {
                    db.delete_all(to_delete, &mut connection).await
                  }

                  if !prs.is_empty() {
                    db.insert_all(prs, &mut connection).await
                  }
                })
                .await
            }
          }
          slack_models::MessageUpdate::MessageDeleted {
            channel,
            channel_type: _,
            event_ts: _,
            previous_message,
          } => {
            let to_delete = ToDelete::from_message(&previous_message.text.0, &channel.0);
            if !to_delete.is_empty() {
              info!("Extracted to_delete: {:?}", to_delete);
              let db = state.db();
              db.transactionally(|mut connection| async move {
                db.delete_all(to_delete, &mut connection).await
              })
              .await
            }
          }
        },
      }
      Json(slack_models::Response::Ok)
    }
  }
}
