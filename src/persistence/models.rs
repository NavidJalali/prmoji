use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct PullRequestTable {
  pub id: Uuid,
  pub url: String,
  pub inserted_at: DateTime<Utc>,
  pub channel: String,
  pub timestamp: String,
}
