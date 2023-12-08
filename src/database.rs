use std::{collections::HashMap, sync::Arc};

use futures::Future;
use tokio::sync::RwLock;

use crate::{
  models::{Channel, ToDelete, PR},
  url_extractor::PrUrl,
};

#[async_trait::async_trait]
pub trait Database<Connection> {
  async fn transactionally<T, F, Fut>(&self, f: F) -> T
  where
    F: FnOnce(Connection) -> Fut + Send + Sync,
    Fut: Future<Output = T> + Send;
  async fn list(&self, connection: &mut Connection) -> Vec<PR>;
  async fn insert_all(&self, prs: Vec<PR>, connection: &mut Connection) -> ();
  async fn delete_all(&self, ids: Vec<ToDelete>, connection: &mut Connection) -> ();
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
impl Database<()> for InMemoryDB {
  async fn transactionally<T, F, Fut>(&self, f: F) -> T
  where
    F: FnOnce(()) -> Fut + Send + Sync,
    Fut: Future<Output = T> + Send,
  {
    f(()).await
  }

  async fn list(&self, _: &mut ()) -> Vec<PR> {
    let items = self.items.read().await;
    items.values().cloned().collect()
  }

  async fn insert_all(&self, prs: Vec<PR>, _: &mut ()) {
    let mut items = self.items.write().await;
    for pr in prs {
      let key = (pr.url.clone(), pr.channel.clone());
      items.insert(key, pr);
    }
  }

  async fn delete_all(&self, to_deletes: Vec<ToDelete>, _: &mut ()) {
    let mut items = self.items.write().await;
    for to_delete in to_deletes {
      let key = (to_delete.url.clone(), to_delete.channel.clone());
      items.remove(&key);
    }
  }
}
