use std::{collections::HashMap, sync::Arc};

use futures::Future;
use tokio::sync::RwLock;

use crate::models::{PrId, PrUrl, ToDelete, ToInsert, PR};

#[async_trait::async_trait]
pub trait PrRepository<Connection> {
  async fn transactionally<T, F, Fut>(&self, f: F) -> T
  where
    F: FnOnce(Connection) -> Fut + Send + Sync,
    Fut: Future<Output = T> + Send;
  async fn list(&self, connection: &mut Connection) -> Vec<PR>;
  async fn get_by_url(&self, url: PrUrl, connection: &mut Connection) -> Vec<PR>;
  async fn insert_all(&self, prs: ToInsert, connection: &mut Connection) -> ();
  async fn delete_all(&self, ids: ToDelete, connection: &mut Connection) -> ();
}

#[derive(Clone)]
pub struct InMemoryPrRepository {
  items: Arc<RwLock<HashMap<PrUrl, Vec<PR>>>>,
}

impl InMemoryPrRepository {
  pub fn new() -> Self {
    Self {
      items: Arc::new(RwLock::new(HashMap::new())),
    }
  }
}

#[async_trait::async_trait]
impl PrRepository<()> for InMemoryPrRepository {
  async fn transactionally<T, F, Fut>(&self, f: F) -> T
  where
    F: FnOnce(()) -> Fut + Send + Sync,
    Fut: Future<Output = T> + Send,
  {
    f(()).await
  }

  async fn list(&self, _: &mut ()) -> Vec<PR> {
    self
      .items
      .read()
      .await
      .values()
      .flatten()
      .cloned()
      .collect()
  }

  async fn get_by_url(&self, url: PrUrl, _: &mut ()) -> Vec<PR> {
    let items = self.items.read().await;
    items.get(&url).cloned().unwrap_or_default()
  }

  async fn insert_all(&self, to_insert: ToInsert, _: &mut ()) {
    let ToInsert {
      urls,
      inserted_at,
      channel,
      timestamp,
    } = to_insert;

    let mut items = self.items.write().await;

    for url in urls {
      let pr = PR {
        id: PrId::random(),
        url: url.clone(),
        inserted_at: inserted_at.clone(),
        channel: channel.clone(),
        timestamp: timestamp.clone(),
      };

      let key = url;
      let vec = items.entry(key).or_insert_with(Vec::new);
      vec.push(pr);
    }
  }

  async fn delete_all(&self, to_delete: ToDelete, _: &mut ()) {
    let ToDelete {
      urls,
      channel,
      timestamp,
    } = to_delete;

    let mut items = self.items.write().await;

    for url in urls {
      let key = url;
      let vec = items.entry(key).or_insert_with(Vec::new);
      *vec = vec
        .iter()
        .filter(|pr| !(pr.channel != channel && pr.timestamp != timestamp))
        .cloned()
        .collect();
    }
  }
}
