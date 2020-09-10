use crate::OSClock;
use std::io::Error;
use std::time::Duration;

mod bindings;

use bindings::{
    mach_port_t, mach_thread_self, thread_basic_info, thread_info, thread_info_t, KERN_SUCCESS,
    THREAD_BASIC_INFO, THREAD_BASIC_INFO_COUNT,
};

use std::mem::MaybeUninit;

pub struct MachThreadClock {
    thread: mach_port_t,
}

impl OSClock for MachThreadClock {
    fn get_time(&self) -> Result<Duration, Error> {
        let mut info = MaybeUninit::<thread_basic_info>::uninit();
        let mut count = THREAD_BASIC_INFO_COUNT;

        if unsafe {
            thread_info(
                self.thread,
                THREAD_BASIC_INFO,
                info.as_mut_ptr() as thread_info_t,
                &mut count,
            )
        } != KERN_SUCCESS
        {
            return Err(Error::last_os_error());
        }

        let info: thread_basic_info = unsafe { info.assume_init() };

        let user_time = Duration::new(
            info.user_time.seconds as u64,
            (info.user_time.microseconds * 1000) as u32,
        );

        let system_time = Duration::new(
            info.system_time.seconds as u64,
            (info.system_time.microseconds * 1000) as u32,
        );

        return Ok(user_time + system_time);
    }
}

pub fn clock_for_current_thread() -> Result<MachThreadClock, Error> {
    let thread = unsafe { mach_thread_self() };

    Ok(MachThreadClock { thread })
}
