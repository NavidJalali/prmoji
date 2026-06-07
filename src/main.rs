use std::ops::Deref;

use app_state::AppState;
use axum::{middleware::from_fn_with_state, routing::post, Router};
use std::io::*;
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

  let debug = Router::<S>::new().route("/debug", post(api::debug));

  Router::new()
    .merge(github)
    .merge(slack)
    .merge(debug)
    .with_state(state)
    .layer(TraceLayer::new_for_http())
}

#[tokio::main]
async fn main() -> Result<()> {
  let config = Configuration::new().map_err(|config_error| {
    Error::other(format!("Failed to load configuration: {:?}", config_error))
  })?;

  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::DEBUG)
    .finish();

  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

  let state = LiveState::new(&config).await;

  sqlx::migrate!("./migrations")
    .run(state.pr_repository().pool.deref())
    .await
    .map_err(|migrate_error| {
      Error::other(format!("Failed to run migrations: {:?}", migrate_error))
    })?;

  let app = make_router(state);

  let listener = TcpListener::bind(&config.server.addr()).await?;

  info!("Listening on {:?}", listener.local_addr()?);

  axum::serve(listener, app.into_make_service()).await?;

  Ok(())
}
