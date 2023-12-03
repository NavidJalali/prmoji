use crate::database::Database;
use crate::models::*;
use crate::slack;
use crate::AppState;
use axum::extract::State;
use axum::Json;
use tracing::info;

pub async fn list<S: AppState>(state: State<S>) -> Json<Vec<PR>> {
  let db = state.db();
  let prs = db
    .transactionally(|mut connection| async move { db.list(&mut connection).await })
    .await;
  Json(prs)
}

pub async fn debug<S: AppState>(Json(payload): Json<serde_json::Value>) {
  info!("Received payload: {}", payload.to_string());
}

pub async fn event<S: AppState>(
  state: State<S>,
  Json(payload): Json<slack::WebookCallback>,
) -> Json<slack::Response> {
  info!("Received payload: {:?}", payload);
  match payload {
    slack::WebookCallback::UrlVerification { challenge, .. } => {
      Json(slack::Response::ChallengeReply { challenge })
    }
    slack::WebookCallback::EventCallback { event, .. } => {
      match event {
        slack::Event::Create(message) => {
          let prs = PR::from_message(&message.text.0, &message.channel.0, state.clock());
          let db = state.db();

          if !prs.is_empty() {
            info!("Extracted prs: {:?}", prs);
            db.transactionally(|mut connection| async move {
              db.insert_all(prs, &mut connection).await
            })
            .await;
          }
        }
        slack::Event::Update(update) => match update {
          slack::MessageUpdate::MessageChanged {
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
                  db.delete_all(to_delete, &mut connection).await;
                  db.insert_all(prs, &mut connection).await;
                })
                .await;
            }
          }
          slack::MessageUpdate::MessageDeleted {
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
      Json(slack::Response::Ok)
    }
  }
}
