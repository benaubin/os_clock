use crate::Clock;
use std::io::Error;
use std::time::Duration;

mod bindings;

mod thread;
use thread::Thread;
pub struct ThreadCPUClock(Thread);

impl Clock for ThreadCPUClock {
    fn get_time(&self) -> Result<Duration, Error> {
        let info = self.0.get_basic_info()?;
        let time = Duration::from(info.user_time) + Duration::from(info.system_time);
        Ok(time)
    }
}

pub fn cpu_clock_for_current_thread() -> Result<ThreadCPUClock, Error> {
    Ok(ThreadCPUClock(Thread::current()))
}
