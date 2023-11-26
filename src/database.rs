use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::{
  models::{Channel, ToDelete, PR},
  url_extractor::PrUrl,
};

#[async_trait::async_trait]
pub trait Database {
  async fn list(&self) -> Vec<PR>;
  async fn upsert_all(&self, prs: Vec<PR>) -> ();
  async fn delete_all(&self, ids: Vec<ToDelete>) -> ();
}

#[derive(Clone)]
pub struct InMemoryDB {
  items: Arc<RwLock<HashMap<(PrUrl, Channel), PR>>>,
}

impl InMemoryDB {
  pub fn new() -> Self {
    Self {
      items: Arc::new(RwLock::new(HashMap::new())),
    }
  }
}

#[async_trait::async_trait]
impl Database for InMemoryDB {
  async fn list(&self) -> Vec<PR> {
    let items = self.items.read().await;
    items.values().cloned().collect()
  }

  async fn upsert_all(&self, prs: Vec<PR>) {
    let mut items = self.items.write().await;
    for pr in prs {
      let key = (pr.url.clone(), pr.channel.clone());
      items.insert(key, pr);
    }
  }

  async fn delete_all(&self, to_deletes: Vec<ToDelete>) {
    let mut items = self.items.write().await;
    for to_delete in to_deletes {
      let key = (to_delete.url.clone(), to_delete.channel.clone());
      items.remove(&key);
    }
  }
}
