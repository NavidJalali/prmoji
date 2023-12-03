use app_state::AppState;
use axum::{
  routing::{get, post},
  Router,
};
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::app_state::TestState;

mod app_state;
mod clock;
mod database;
mod github;
mod handlers;
mod models;
mod slack;
mod url_extractor;

enum Emoji {
  Merged,
  Deleted,
  Approved,
  Comments,
  Custom(String),
}

pub fn make_router<S: AppState>(state: S) -> Router {
  Router::new()
    .route("/", post(handlers::event::<S>))
    .route("/", get(handlers::list::<S>))
    .route("/github", post(handlers::debug::<S>))
    .with_state(state)
}

#[tokio::main]
async fn main() {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::DEBUG)
    .finish();

  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

  let state = TestState::new();

  let app = make_router(state);

  let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

  info!("Listening on {:?}", listener.local_addr().unwrap());

  axum::serve(listener, app.into_make_service())
    .await
    .unwrap();
}
