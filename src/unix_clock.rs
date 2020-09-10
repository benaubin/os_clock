pub struct UnixClock {
  clock_id: clockid_t
}

impl OSClock for UnixClock {
  fn get_time(&self) -> Result<Duration, Error> {
    let mut timespec = timespec {
      tv_sec: 0,
      tv_nsec: 0,
    };

    if unsafe { clock_gettime(self.clock_id, &mut timespec) } == -1 {
      return Err(Error::last_os_error());
    }

    Ok(Duration::new(
      timespec.tv_sec as u64, // docs state valid values are ≥ 0
      timespec.tv_nsec as u32 // we're lucky here: the docs state that valid values are [0, 999999999]
    ))
  }
}
