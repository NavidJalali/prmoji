use std::ops::Deref;
use std::sync::Arc;

use sqlx::Postgres;
use tracing::error;

use crate::models::{PrUrl, ToDelete, ToInsert, PR};

use crate::persistence::models::PullRequestTable;

#[async_trait::async_trait]
pub trait PrRepository {
  async fn list(&self) -> Vec<PR>;
  async fn get_by_url(&self, url: PrUrl) -> Vec<PR>;
  async fn insert_all(&self, to_insert: ToInsert) -> ();
  async fn delete_all(&self, to_delete: ToDelete) -> ();
  async fn update(&self, to_insert: ToInsert, to_delete: ToDelete) -> ();
}

#[derive(Clone)]
pub struct LivePrRepository {
  pub pool: Arc<sqlx::Pool<Postgres>>,
}

impl LivePrRepository {
  pub async fn new(config: &crate::config::Database) -> Self {
    let pool = Arc::new(
      sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.pool_size)
        .connect(&config.url())
        .await
        .expect("Failed to connect to Postgres"),
    );

    Self { pool }
  }
}

#[async_trait::async_trait]
impl PrRepository for LivePrRepository {
  async fn list(&self) -> Vec<PR> {
    let query = sqlx::query_as::<_, PullRequestTable>("select * from pull_requests");
    let prs = query.fetch_all(self.pool.as_ref()).await.unwrap();
    prs.into_iter().map(|pr| pr.into()).collect()
  }

  async fn get_by_url(&self, url: PrUrl) -> Vec<PR> {
    let query = sqlx::query_as::<_, PullRequestTable>("select * from pull_requests where url = $1");
    let prs = query
      .bind(url.0)
      .fetch_all(self.pool.as_ref())
      .await
      .unwrap();
    prs.into_iter().map(|pr| pr.into()).collect()
  }

  async fn insert_all(&self, to_insert: ToInsert) -> () {
    let mut txn = self.pool.begin().await.unwrap();
    let ToInsert {
      channel,
      timestamp,
      urls,
      inserted_at,
    } = to_insert;
    let prepared = r"insert into pull_requests (id, url, inserted_at, channel, timestamp) values ($1, $2, $3, $4, $5)";

    for url in urls {
      let result = sqlx::query(prepared)
        .bind(uuid::Uuid::new_v4())
        .bind(url.0)
        .bind(inserted_at)
        .bind(channel.0.clone())
        .bind(timestamp.0.clone())
        .execute(&mut *txn)
        .await;

      match result {
        Ok(_) => (),
        Err(e) => {
          error!("Failed to insert: {}", e);
          txn.rollback().await.unwrap();
          return;
        }
      };
    }

    txn.commit().await.unwrap();
  }

  async fn delete_all(&self, to_delete: ToDelete) -> () {
    if to_delete.urls.is_empty() {
      return ();
    } else {
      let ToDelete {
        urls,
        channel,
        timestamp,
      } = to_delete;

      sqlx::query(
        "delete from pull_requests where channel = $1 and timestamp = $2 and url in ($3)",
      )
      .bind(channel.0)
      .bind(timestamp.0)
      .bind(
        urls
          .into_iter()
          .map(|url| url.0)
          .collect::<Vec<_>>()
          .join(","),
      )
      .execute(self.pool.deref())
      .await
      .expect("Failed to delete");
    };
  }

  async fn update(&self, to_insert: ToInsert, to_delete: ToDelete) -> () {
    let mut txn = self.pool.begin().await.unwrap();

    if !to_delete.urls.is_empty() {
      let ToDelete {
        urls,
        channel,
        timestamp,
      } = to_delete;

      let result = sqlx::query(
        "delete from pull_requests where channel = $1 and timestamp = $2 and url in ($3)",
      )
      .bind(channel.0)
      .bind(timestamp.0)
      .bind(
        urls
          .into_iter()
          .map(|url| url.0)
          .collect::<Vec<_>>()
          .join(","),
      )
      .execute(&mut *txn)
      .await;

      match result {
        Ok(_) => (),
        Err(e) => {
          error!("Failed to delete: {}", e);
          txn.rollback().await.unwrap();
          return;
        }
      };
    }

    if !to_insert.urls.is_empty() {
      let ToInsert {
        channel,
        timestamp,
        urls,
        inserted_at,
      } = to_insert;
      let prepared = r"insert into pull_requests (id, url, inserted_at, channel, timestamp) values ($1, $2, $3, $4, $5)";

      for url in urls {
        let result = sqlx::query(prepared)
          .bind(uuid::Uuid::new_v4())
          .bind(url.0)
          .bind(inserted_at)
          .bind(channel.0.clone())
          .bind(timestamp.0.clone())
          .execute(&mut *txn)
          .await;

        match result {
          Ok(_) => (),
          Err(e) => {
            error!("Failed to insert: {}", e);
            txn.rollback().await.unwrap();
            return;
          }
        };
      }
    }

    txn.commit().await.unwrap();
  }
}
