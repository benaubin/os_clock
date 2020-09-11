//! Access various operating system clocks (such as per-thread CPU Time, system clock, monotomic, etc) on Unix-family systems.
//!
//! ## Thread clocks:
//!
//! Sendable per-thread CPU clocks are unique to this crate:
//!
//! ```
//! # use std::time::Duration;
//! # use os_clock::{self, Clock};
//! #
//! let clock = os_clock::cpu_clock_for_current_thread().unwrap();
//!
//! let start_time = clock.get_time().unwrap();
//! // Do some work for 5ms...
//! # loop {
//! #     if clock.get_time().unwrap() > (start_time + Duration::from_millis(5)) {
//! #         break;
//! #     }
//! # }
//! assert!(clock.get_time().unwrap() > start_time + Duration::from_millis(5));
//!
//! // Notably, a clock for the CPU time of one thread can be accessed from another thread:
//! std::thread::spawn(move || {
//!     assert!(clock.get_time().unwrap() > Duration::from_millis(5));
//!
//!     let self_clock = os_clock::cpu_clock_for_current_thread().unwrap();
//!     assert!(self_clock.get_time().unwrap() < Duration::from_millis(1));
//! })
//! .join()
//! # .unwrap();
//!
//! // Clocks count from the thread's spawn time
//! let new_clock = os_clock::cpu_clock_for_current_thread().unwrap();
//! assert!(new_clock.get_time().unwrap() > Duration::from_millis(5));
//! ```

use std::io::Error;
use std::time::Duration;

#[cfg_attr(any(target_os = "macos", target_os = "ios"), path = "mach/mod.rs")]
#[allow(unused_attributes)] // in order to allow #[path = "pthread.rs"] to work
#[path = "pthread.rs"]
mod os;

pub use os::{cpu_clock_for_current_thread, ThreadCPUClock};

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use os::Thread;

mod posix_clock;
pub use posix_clock::{
    get_current_thread_cpu_time, PosixClock, MONOTONIC_CLOCK, PROCESS_CLOCK, REALTIME_CLOCK,
};

pub trait Clock: Sized + Send {
    /// Get the current time value of the clock.
    ///
    /// Note that the meaning of the `Duration` differs depending on implementation.
    /// Sometimes the clock represents CPU time, sometimes wall time, etc.
    fn get_time(&self) -> Result<Duration, Error>;
}

#[cfg(test)]
mod tests {
    use super::{cpu_clock_for_current_thread, Clock};

    #[test]
    fn valid_measurement() {
        let clock = cpu_clock_for_current_thread().unwrap();

        let mut samples = std::iter::repeat::<()>(())
            .map(|_| clock.get_time().unwrap())
            .step_by(50000);

        let mut last_time = samples.next().unwrap();

        let samples = samples
            .take(5)
            .map(|this_time| {
                assert!(this_time > last_time);
                let diff = (this_time - last_time).as_secs_f64();
                last_time = this_time;
                diff
            })
            .collect::<Vec<f64>>();

        let avg = samples.iter().sum::<f64>() / (samples.len() as f64);

        let mean_abs_dev_scaled = samples
            .iter()
            .map(|sample| (sample - avg).abs())
            .sum::<f64>()
            / (samples.len() as f64)
            / avg;

        println!(
            "
durations of timing 50000 samples
==================================
{:#?}
----------------------------------
avg: {}, mad scaled: {}",
            samples, avg, mean_abs_dev_scaled
        );

        assert!(mean_abs_dev_scaled < 0.1); // test that samples are on average within 10% of the mean
    }
}
