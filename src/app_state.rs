use crate::clock;
use crate::database;
use crate::slack;

pub trait AppState: Clone + Send + Sync + 'static {
  type ConnectionType: Sync + Send;
  type PrRepo: database::PrRepository<Self::ConnectionType> + Sync + Send;
  type SlackClient: slack::SlackClient + Sync + Send;
  type Clock: clock::Clock + Sync + Send;

  fn pr_repository(&self) -> &Self::PrRepo;
  fn clock(&self) -> &Self::Clock;
  fn slack_client(&self) -> &Self::SlackClient;
}

#[derive(Clone)]
pub struct TestState {
  pub db: database::InMemoryPrRepository,
  pub clock: clock::LiveClock,
  pub slack_client: slack::LiveSlackClient,
}

impl TestState {
  pub fn new() -> Self {
    Self {
      db: database::InMemoryPrRepository::new(),
      clock: clock::LiveClock,
      slack_client: slack::LiveSlackClient::new(),
    }
  }
}

impl AppState for TestState {
  type ConnectionType = ();
  type PrRepo = database::InMemoryPrRepository;
  fn pr_repository(&self) -> &Self::PrRepo {
    &self.db
  }

  type Clock = clock::LiveClock;
  fn clock(&self) -> &Self::Clock {
    &self.clock
  }

  type SlackClient = slack::LiveSlackClient;
  fn slack_client(&self) -> &Self::SlackClient {
    &self.slack_client
  }
}
