use crate::clock;
use crate::database;

pub trait AppState: Clone + Send + Sync + 'static {
    type DB: database::Database + Sync + Send;
    type Clock: clock::Clock + Sync + Send;

    fn db(&self) -> &Self::DB;
    fn clock(&self) -> &Self::Clock;
}

#[derive(Clone)]
pub struct TestState {
    pub db: database::InMemoryDB,
    pub clock: clock::LiveClock,
}

impl TestState {
    pub fn new() -> Self {
        Self {
            db: database::InMemoryDB::new(),
            clock: clock::LiveClock,
        }
    }
}

impl AppState for TestState {
    type DB = database::InMemoryDB;
    fn db(&self) -> &Self::DB {
        &self.db
    }

    type Clock = clock::LiveClock;
    fn clock(&self) -> &Self::Clock {
        &self.clock
    }
}
