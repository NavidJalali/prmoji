use std::hash::Hash;

use crate::{
  persistence::models::PullRequestTable,
  slack::models::{Channel, Timestamp},
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct PrUrl(pub String);

impl From<&str> for PrUrl {
  fn from(s: &str) -> Self {
    PrUrl(s.to_string())
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrId(pub Uuid);

impl Hash for PrId {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.hash(state)
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct PR {
  pub id: PrId,
  pub url: PrUrl,
  pub inserted_at: DateTime<Utc>,
  pub channel: Channel,
  pub timestamp: Timestamp,
}

impl From<PullRequestTable> for PR {
  fn from(pr: PullRequestTable) -> Self {
    Self {
      id: PrId(pr.id),
      url: PrUrl(pr.url),
      inserted_at: pr.inserted_at,
      channel: Channel(pr.channel),
      timestamp: Timestamp(pr.timestamp),
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct ToDelete {
  pub urls: Vec<PrUrl>,
  pub channel: Channel,
  pub timestamp: Timestamp,
}

impl ToDelete {
  pub fn new(urls: Vec<PrUrl>, channel: Channel, timestamp: Timestamp) -> Self {
    Self {
      urls,
      channel,
      timestamp,
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct ToInsert {
  pub urls: Vec<PrUrl>,
  pub inserted_at: DateTime<Utc>,
  pub channel: Channel,
  pub timestamp: Timestamp,
}

impl ToInsert {
  pub fn new(
    urls: Vec<PrUrl>,
    channel: Channel,
    timestamp: Timestamp,
    inserted_at: DateTime<Utc>,
  ) -> Self {
    Self {
      urls,
      inserted_at,
      channel,
      timestamp,
    }
  }
}
