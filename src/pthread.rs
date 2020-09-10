use libc::{clock_gettime, clockid_t, pthread_self, pthread_t, timespec};
use std::convert::{TryFrom, TryInto};
use std::io::Error;
use std::os::raw::{c_int, c_long};
use std::time::Duration;

use crate::OSClock;

#[link(name = "pthread")]
extern "C" {
    fn pthread_getcpuclockid(thread: pthread_t, clock_id: *mut clockid_t) -> c_int;
}

fn clock_for_current_thread() -> Result<UnixClock, Error> {
    let mut clock_id = 0 as clockid_t;

    if unsafe { pthread_getcpuclockid(pthread_self(), &mut clock_id) } != 0 {
        return Err(Error::last_os_error());
    }

    return Ok(UnixClock { clock_id });
}
