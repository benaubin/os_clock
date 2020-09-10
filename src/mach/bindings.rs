#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/mach_bindings.rs"));

use std::mem::size_of;
use std::time::Duration;

pub const THREAD_BASIC_INFO: mach_msg_type_number_t = 3;
pub const THREAD_BASIC_INFO_COUNT: mach_msg_type_number_t =
    (size_of::<thread_basic_info_data_t>() / size_of::<natural_t>()) as u32;
pub const KERN_SUCCESS: i32 = 0;

impl From<time_value> for Duration {
    fn from(time_value: time_value) -> Duration {
        Duration::new(
            time_value.seconds as u64,
            (time_value.microseconds * 1000) as u32,
        )
    }
}
