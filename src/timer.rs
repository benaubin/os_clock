use std::io::Result;
use std::time::Duration;

use super::Clock;

/// Helper to measure time elapsed
///
/// Get a timer with `Clock.start_timer()`
pub struct Timer<'c, C: Clock> {
  pub start: Duration,
  pub clock: &'c C,
}

impl<'c, C: Clock> Timer<'c, C> {
  /// Returns the ellapsed time since `start`
  pub fn elapsed(&self) -> Result<Duration> {
    let end = self.clock.get_time()?;
    return Ok(end - self.start);
  }

  /// Drops the timer and returns the ellapsed time since `start`
  #[inline(always)]
  pub fn end(self) -> Result<Duration> {
    return self.elapsed();
  }
}
