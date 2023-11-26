use std::hash::Hash;

use crate::{
  clock::Clock,
  url_extractor::{extract_pr_urls, PrUrl},
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrId(pub Uuid);

impl PrId {
  pub fn random() -> Self {
    Self(Uuid::new_v4())
  }
}

impl Hash for PrId {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.hash(state)
  }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
pub struct Channel(pub String);

#[derive(Debug, Clone, Serialize)]
pub struct PR {
  pub id: PrId,
  pub url: PrUrl,
  pub inserted_at: DateTime<Utc>,
  pub channel: Channel,
}

impl PR {
  pub fn from_message(message: &String, channel: &String, clock: &impl Clock) -> Vec<PR> {
    let urls = extract_pr_urls(&message);
    urls
      .into_iter()
      .map(|url| PR {
        id: PrId::random(),
        url: url,
        inserted_at: clock.now(),
        channel: Channel(channel.clone()),
      })
      .collect::<Vec<PR>>()
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct ToDelete {
  pub url: PrUrl,
  pub channel: Channel,
}

impl ToDelete {
  pub fn from_message(message: &String, channel: &String) -> Vec<ToDelete> {
    let urls = extract_pr_urls(&message);
    urls
      .into_iter()
      .map(|url| ToDelete {
        url: url,
        channel: Channel(channel.clone()),
      })
      .collect::<Vec<ToDelete>>()
  }
}
