use std::ops::Deref;

use app_state::AppState;
use axum::{
  routing::{get, post},
  Router,
};
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::{app_state::TestState, config::Configuration};

mod app_state;
mod clock;
mod config;
mod github;
mod handlers;
mod models;
mod persistence;
mod slack;
mod url_extractor;

pub fn make_router<S: AppState>(state: S) -> Router {
  Router::new()
    .route("/slack", post(handlers::handle_slack_event::<S>))
    .route("/", get(handlers::list::<S>))
    .route("/github", post(handlers::handle_github_event::<S>))
    .with_state(state)
}

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  let config = Configuration::new().unwrap();

  println!("{:?}", config);

  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::DEBUG)
    .finish();

  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

  let state = TestState::new(&config).await;

  sqlx::migrate!("./migrations")
    .run(state.pr_repository().pool.deref())
    .await
    .unwrap();

  let app = make_router(state);

  let listener = TcpListener::bind(&config.server.addr()).await.unwrap();

  info!("Listening on {:?}", listener.local_addr().unwrap());

  axum::serve(listener, app.into_make_service())
    .await
    .unwrap();
}
