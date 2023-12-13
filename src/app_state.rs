use crate::clock;
use crate::config::Configuration;
use crate::persistence::pr_repository;
use crate::slack;

pub trait AppState: Clone + Send + Sync + 'static {
  type PrRepo: pr_repository::PrRepository + Sync + Send;
  type SlackClient: slack::SlackClient + Sync + Send;
  type Clock: clock::Clock + Sync + Send;

  fn pr_repository(&self) -> &Self::PrRepo;
  fn clock(&self) -> &Self::Clock;
  fn slack_client(&self) -> &Self::SlackClient;
  fn config(&self) -> &Configuration;
}

#[derive(Clone)]
pub struct TestState {
  pub clock: clock::LiveClock,
  pub slack_client: slack::LiveSlackClient,
  pub config: Configuration,
  pub pr_repository: pr_repository::LivePrRepository,
}

impl TestState {
  pub async fn new(config: &Configuration) -> Self {
    Self {
      clock: clock::LiveClock,
      slack_client: slack::LiveSlackClient::new(&config.slack),
      config: config.clone(),
      pr_repository: pr_repository::LivePrRepository::new(&config.database).await,
    }
  }
}

impl AppState for TestState {
  type PrRepo = pr_repository::LivePrRepository;
  fn pr_repository(&self) -> &Self::PrRepo {
    &self.pr_repository
  }

  type Clock = clock::LiveClock;
  fn clock(&self) -> &Self::Clock {
    &self.clock
  }

  type SlackClient = slack::LiveSlackClient;
  fn slack_client(&self) -> &Self::SlackClient {
    &self.slack_client
  }

  fn config(&self) -> &Configuration {
    &self.config
  }
}
