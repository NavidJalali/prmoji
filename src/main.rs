use std::ops::Deref;

use app_state::AppState;
use axum::{middleware::from_fn_with_state, routing::post, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::{app_state::LiveState, config::Configuration};

mod api;
mod app_state;
mod clock;
mod config;
mod github;
mod models;
mod persistence;
mod slack;
mod url_extractor;

pub fn make_router<S: AppState>(state: S) -> Router {
  let github = Router::new()
    .route("/github", post(api::handle_github_webhook::<S>))
    .route_layer(from_fn_with_state(
      state.clone(),
      api::auth::authenticate_github_webhook::<S>,
    ));

  let slack = Router::new()
    .route("/slack", post(api::handle_slack_webhook::<S>))
    .route_layer(from_fn_with_state(
      state.clone(),
      api::auth::authenticate_slack_webhook::<S>,
    ));

  Router::new()
    .merge(github)
    .merge(slack)
    .with_state(state)
    .layer(TraceLayer::new_for_http())
}

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  let config = Configuration::new().unwrap();

  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::DEBUG)
    .finish();

  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

  let state = LiveState::new(&config).await;

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
