use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::models::{PrId, PR};

#[async_trait::async_trait]
pub trait Database {
    async fn list(&self) -> Vec<PR>;
    async fn get(&self, id: PrId) -> Option<PR>;
    async fn upsert_all(&self, prs: Vec<PR>) -> ();
}

#[derive(Clone)]
pub struct InMemoryDB {
    items: Arc<RwLock<HashMap<PrId, PR>>>,
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

    async fn get(&self, id: PrId) -> Option<PR> {
        let items = self.items.read().await;
        items.get(&id).cloned()
    }

    async fn upsert_all(&self, prs: Vec<PR>) {
        let mut items = self.items.write().await;
        for pr in prs {
            items.insert(pr.id.clone(), pr);
        }
    }
}
