use libc::{clock_gettime, clockid_t, timespec};
use std::io::Error;
use std::time::Duration;

use crate::Clock;

impl Clock for clockid_t {
  fn get_time(&self) -> Result<Duration, Error> {
    let mut timespec = timespec {
      tv_sec: 0,
      tv_nsec: 0,
    };

    if unsafe { clock_gettime(self.clone(), &mut timespec) } == -1 {
      return Err(Error::last_os_error());
    }

    Ok(Duration::new(
      timespec.tv_sec as u64,  // docs state valid values are ≥ 0
      timespec.tv_nsec as u32, // we're lucky here: the docs state that valid values are [0, 999999999]
    ))
  }
}
