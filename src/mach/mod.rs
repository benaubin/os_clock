use std::io::Error;

mod bindings;

mod thread;
pub use thread::{Thread, ThreadCPUClock};

// Get the CPU clock for the current thread
pub fn cpu_clock_for_current_thread() -> Result<ThreadCPUClock, Error> {
    Ok(Thread::current().into())
}
