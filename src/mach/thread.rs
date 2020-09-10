use std::io::Error;

use super::bindings::{
    mach_port_t, mach_thread_self, thread_basic_info, thread_info, thread_info_t, KERN_SUCCESS,
    THREAD_BASIC_INFO, THREAD_BASIC_INFO_COUNT,
};

use std::mem::MaybeUninit;

pub struct Thread(mach_port_t);

impl Thread {
    pub fn get_basic_info(&self) -> Result<thread_basic_info, Error> {
        let mut info = MaybeUninit::<thread_basic_info>::zeroed(); // TODO: does this really need to be zeroed?
        let mut count = THREAD_BASIC_INFO_COUNT;

        if unsafe {
            thread_info(
                self.0,
                THREAD_BASIC_INFO,
                info.as_mut_ptr() as thread_info_t,
                &mut count,
            )
        } != KERN_SUCCESS
        {
            return Err(Error::last_os_error());
        }

        Ok(unsafe { info.assume_init() })
    }

    pub fn current() -> Thread {
        Thread(unsafe { mach_thread_self() })
    }
}
