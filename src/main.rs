use app_state::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::app_state::TestState;

mod app_state;
mod clock;
mod database;
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

#[async_trait::async_trait]
trait App {
    async fn handle_message(message: String) -> Result<(), String>;
}

#[async_trait::async_trait]
trait Slack {
    async fn add_emoji() -> Result<(), String>;
}

mod handlers {
    use crate::clock::Clock;
    use crate::database::Database;
    use crate::models::*;
    use crate::slack;
    use crate::url_extractor;
    use crate::AppState;
    use axum::extract::State;
    use axum::Json;
    use tracing::info;

    pub async fn list<S: AppState>(state: State<S>) -> Json<Vec<PR>> {
        let prs = state.db().list().await;
        Json(prs)
    }

    pub async fn debug<S: AppState>(Json(payload): Json<serde_json::Value>) {
        info!("Received payload: {}", payload.to_string());
    }

    fn extract_prs(message: &String, channel: &String, clock: &impl Clock) -> Vec<PR> {
        let urls = url_extractor::extract_pr_urls(&message);
        urls.into_iter()
            .map(|url| PR {
                id: PrId::random(),
                url: url,
                inserted_at: clock.now(),
                channel: Channel(channel.clone()),
            })
            .collect::<Vec<PR>>()
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
                        let prs = extract_prs(&message.text.0, &message.channel.0, state.clock());
                        if !prs.is_empty() {
                            info!("Extracted prs: {:?}", prs);
                            state.db().upsert_all(prs).await;
                        }
                    }
                    slack::Event::Update(update) => match update {
                        slack::MessageUpdate::MessageChanged {
                            message,
                            previous_message: _,
                            channel,
                            channel_type: _,
                            event_ts: _,
                        } => {
                            info!("Received message update: {:?}", message);
                            let prs = extract_prs(&message.text.0, &channel.0, state.clock());
                            if !prs.is_empty() {
                                info!("Extracted prs: {:?}", prs);
                                state.db().upsert_all(prs).await;
                            }
                        }
                        slack::MessageUpdate::MessageDeleted {
                            channel: _,
                            channel_type: _,
                            event_ts: _,
                            previous_message: _,
                        } => {
                            info!("Received message deletion");
                        }
                    },
                }
                Json(slack::Response::Ok)
            }
        }
    }
}

pub fn make_router<S: AppState>(state: S) -> Router {
    Router::new()
        .route("/", post(handlers::event::<S>))
        .route("/", get(handlers::list::<S>))
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
