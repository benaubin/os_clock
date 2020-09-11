use std::io::{Error, Result};
use std::time::Duration;

use super::bindings::{
    mach_port_t, mach_thread_self, thread_basic_info, thread_info, thread_info_t, KERN_SUCCESS,
    THREAD_BASIC_INFO, THREAD_BASIC_INFO_COUNT,
};

use std::mem::MaybeUninit;

use crate::Clock;

/// Information about a thread on the Mach kernel.
///
/// Unsafe to initialize directly, unless known to be a valid thread.
/// Access the current thread with Thread::current()
pub struct Thread(mach_port_t);

impl Thread {
    /// Get basic info about a thread
    pub fn get_basic_info(&self) -> Result<thread_basic_info> {
        let mut info = MaybeUninit::<thread_basic_info>::zeroed(); // TODO: does this really need to be zeroed?

        let s = unsafe {
            // Unsafe for FFI call to get thread info, FFI call upholds invariants
            thread_info(
                self.0,
                THREAD_BASIC_INFO,
                info.as_mut_ptr() as thread_info_t,
                &mut THREAD_BASIC_INFO_COUNT,
            )
        };

        if s != KERN_SUCCESS {
            return Err(Error::last_os_error());
        }

        let info = unsafe { info.assume_init() }; // If thread_info succeeded, info was initialized

        Ok(info)
    }

    /// The amount of time spent running this thread in user mode
    pub fn get_user_time(&self) -> Result<Duration> {
        let info = self.get_basic_info()?;
        let time = Duration::from(info.user_time);
        Ok(time)
    }

    /// The amount of time spent running this thread in system mode
    pub fn get_system_time(&self) -> Result<Duration> {
        let info = self.get_basic_info()?;
        let time = Duration::from(info.system_time);
        Ok(time)
    }

    /// system_time + user_time at a single instant
    pub fn get_cpu_time(&self) -> Result<Duration> {
        let info = self.get_basic_info()?;
        let time = Duration::from(info.user_time) + Duration::from(info.system_time);
        Ok(time)
    }

    /// Get the current thread
    pub fn current() -> Thread {
        Thread(unsafe { mach_thread_self() }) // FFI call, simply returnss the current thread's mach_port_t without changing any global state
    }
}

/// A simple wrapper around Thread to get a Clock of the CPU time of the thread
pub struct ThreadCPUClock(Thread);

impl Clock for ThreadCPUClock {
    /// The amount of system + user CPU time spent in this thread
    fn get_time(&self) -> Result<Duration> {
        self.0.get_cpu_time()
    }
}

impl From<Thread> for ThreadCPUClock {
    fn from(thread: Thread) -> ThreadCPUClock {
        ThreadCPUClock(thread)
    }
}

/// Get a clock for the CPU time of the current thread
///
/// ```
/// use std::io;
/// use os_clock::{self, Clock};
///
/// let clock = os_clock::cpu_clock_for_current_thread().unwrap();
/// let time = clock.get_time().unwrap();
///
/// # let time_2 = clock.get_time().unwrap();
/// #
/// # assert!(time_2 > time);
/// ```
pub fn cpu_clock_for_current_thread() -> Result<ThreadCPUClock> {
    Ok(Thread::current().into())
}
