use std::thread;
use std::time::{Duration, Instant};

pub struct Throttler {
  last: Option<Instant>,
  interval: Duration,
}

impl Throttler {
  pub fn new(interval: Option<Duration>) -> Self {
    Self {
      last: None,
      interval: interval.unwrap_or(Duration::from_secs(1)),
    }
  }

  pub fn tick(&mut self) {
    match self.last {
      Some(last) => {
        while Instant::now() - last < self.interval {
          thread::sleep(Duration::from_millis(100));
        }
      }
      None => {}
    }
    self.last = Some(Instant::now());
  }
}
