use super::Clock;

pub fn cpu_clock_for_current_thread() -> Result<dyn Clock, Error> {
    panic!("Not implemented for target");
}
