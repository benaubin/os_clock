use std::io::Error;
use std::time::Duration;

#[cfg(not(target_family = "unix"))]
compile_error!(
    "Your target platform is not supported. os_clock currently only supports Unix-family systems."
);

#[cfg_attr(any(target_os = "macos", target_os = "ios"), path = "mach/mod.rs")]
#[path = "pthread.rs"]
mod os;

pub use os::cpu_clock_for_current_thread;

mod posix_clock;
pub use posix_clock::{PosixClock, MONOTONIC_CLOCK, PROCESS_CLOCK, REALTIME_CLOCK};

pub trait Clock: Sized + Send {
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
