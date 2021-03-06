use libc::{
    clock_gettime, clockid_t, timespec, CLOCK_MONOTONIC, CLOCK_PROCESS_CPUTIME_ID, CLOCK_REALTIME,
    CLOCK_THREAD_CPUTIME_ID,
};
use std::io::Error;
use std::time::Duration;

use crate::Clock;

/// The POSIX clockid_t represents numerous types of clocks (wall, cpu, etc)
/// However, they share a common API
///
/// https://pubs.opengroup.org/onlinepubs/9699919799/
pub struct PosixClock(clockid_t);

impl PosixClock {
    /// Creating a POSIX clock struct could result in undefined behavior if passed an invalid `clockid_t`
    /// It is safe to call this on a known-valid clockid
    pub unsafe fn from_clockid(clockid: clockid_t) -> PosixClock {
        PosixClock(clockid)
    }
}

// Constant clocks

/// The system-wide realtime clock
pub const REALTIME_CLOCK: PosixClock = PosixClock(CLOCK_REALTIME);
/// The system-wide monotonic clock (defined as a clock that cannot be set and cannot have backwards clock jumps)
pub const MONOTONIC_CLOCK: PosixClock = PosixClock(CLOCK_MONOTONIC);
/// The process-wide cpu-time clock
pub const PROCESS_CLOCK: PosixClock = PosixClock(CLOCK_PROCESS_CPUTIME_ID);

/// The cpu-time clock for the current thread, note that this is __always__ the current thread.
///
/// This is a private api as it is easy to misuse.
/// `get_current_thread_cpu_time` exposes the functionality of this constant.
///
/// Alternatively, use `cpu_clock_for_current_thread` to get a transferable clock for the current thread.
const CURRENT_THREAD_CPUTIME_CLOCK: PosixClock = PosixClock(CLOCK_THREAD_CPUTIME_ID);

/// Get the CPU time of the current thread
///
/// Alternatively, use `cpu_clock_for_current_thread` to get a transferable clock for the current thread.
pub fn get_current_thread_cpu_time() -> Result<Duration, Error> {
    CURRENT_THREAD_CPUTIME_CLOCK.get_time()
}

impl Clock for PosixClock {
    fn get_time(&self) -> Result<Duration, Error> {
        let mut timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };

        // unsafe for use of FFI, does not affect global state
        let s = unsafe { clock_gettime(self.0, &mut timespec) };

        if s == -1 {
            return Err(Error::last_os_error());
        }

        Ok(Duration::new(
            timespec.tv_sec as u64,  // docs state valid values are ≥ 0
            timespec.tv_nsec as u32, // we're lucky here: the docs state that valid values are [0, 999999999]
        ))
    }
}
