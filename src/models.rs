use std::hash::Hash;

use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::url_extractor::PrUrl;

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

#[derive(Debug, Clone, Serialize)]
pub struct Channel(pub String);

#[derive(Debug, Clone, Serialize)]
pub struct PR {
    pub id: PrId,
    pub url: PrUrl,
    pub inserted_at: DateTime<Utc>,
    pub channel: Channel,
}
