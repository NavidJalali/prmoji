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
