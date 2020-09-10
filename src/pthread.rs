use libc::{clockid_t, pthread_self, pthread_t};
use std::io::Error;
use std::os::raw::c_int;

#[link(name = "pthread")]
extern "C" {
    fn pthread_getcpuclockid(thread: pthread_t, clock_id: *mut clockid_t) -> c_int;
}

pub fn cpu_clock_for_current_thread() -> Result<clockid_t, Error> {
    let mut clock_id = 0 as clockid_t;

    if unsafe { pthread_getcpuclockid(pthread_self(), &mut clock_id) } != 0 {
        return Err(Error::last_os_error());
    }
    return Ok(clock_id);
}
