pub trait Clock {
  fn now(&self) -> chrono::DateTime<chrono::Utc>;
}

#[derive(Clone)]
pub struct LiveClock;

impl Clock for LiveClock {
  fn now(&self) -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
  }
}

// Is correct once a day
pub struct FrozenClock(chrono::DateTime<chrono::Utc>);

impl Clock for FrozenClock {
  fn now(&self) -> chrono::DateTime<chrono::Utc> {
    self.0
  }
}
