use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

use crate::slack::models::Emoji;

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
  pub host: String,
  pub port: u16,
}

impl Server {
  pub fn addr(&self) -> String {
    format!("{}:{}", self.host, self.port)
  }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Slack {
  pub bot_token: String,
  signing_secret: String,
}

impl Slack {
  pub fn signing_secret(&self) -> &[u8] {
    self.signing_secret.as_bytes()
  }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Github {
  secret: String,
}

impl Github {
  pub fn secret(&self) -> &[u8] {
    self.secret.as_bytes()
  }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Database {
  pub host: String,
  pub port: u16,
  pub user: String,
  pub password: String,
  pub database: String,
  pub pool_size: u32,
}

impl Database {
  pub fn url(&self) -> String {
    format!(
      "postgres://{}:{}@{}:{}/{}",
      self.user, self.password, self.host, self.port, self.database
    )
  }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Emojis {
  pub merged: String,
  pub closed: String,
  pub approved: String,
  pub commented: String,
  pub changes_requested: String,
}

impl Emojis {
  pub fn get(&self, emoji: Emoji) -> String {
    match emoji {
      Emoji::Merged => self.merged.clone(),
      Emoji::Deleted => self.closed.clone(),
      Emoji::Approved => self.approved.clone(),
      Emoji::Comment => self.commented.clone(),
      Emoji::ChangeRequest => self.changes_requested.clone(),
    }
  }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
  pub server: Server,
  pub slack: Slack,
  pub database: Database,
  pub github: Github,
  pub emojis: Emojis,
}

impl Configuration {
  pub fn new() -> Result<Self, ConfigError> {
    Config::builder()
      .add_source(File::with_name("config/default"))
      .add_source(Environment::default())
      .build()?
      .try_deserialize()
  }
}
