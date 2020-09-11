pub mod bindings;
mod thread;

pub use thread::{cpu_clock_for_current_thread, Thread, ThreadCPUClock};
