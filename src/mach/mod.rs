use std::io::Error;

mod bindings;

mod thread;
pub use thread::{Thread, ThreadCPUClock};

/// Get a clock for the CPU time of the current thread
///
/// ```
/// use std::io;
/// use os_clock::{self, Clock};
///
/// let clock = os_clock::cpu_clock_for_current_thread().unwrap();
/// let time = clock.get_time();
/// ```
pub fn cpu_clock_for_current_thread() -> Result<ThreadCPUClock, Error> {
    Ok(Thread::current().into())
}
