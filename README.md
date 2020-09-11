# os_clock

Access various operating system clocks (such as per-thread CPU Time, system clock, monotomic, etc) on Unix-family systems.

```rs
use os_clock::{self, Clock};

let clock = cpu_clock_for_current_thread();
clock.get_time();
```

Notably, a clock for the CPU time of one thread can be accessed from another thread:

```rs
let clock = cpu_clock_for_current_thread().unwrap();

loop {
    if clock.get_time().unwrap() > Duration::from_millis(5) {
        break;
    }
}

std::thread::spawn(move || {
    assert!(clock.get_time().unwrap() > Duration::from_millis(5));

    let self_clock = cpu_clock_for_current_thread().unwrap();
    assert!(self_clock.get_time().unwrap() < Duration::from_millis(1));
})
.join()
.unwrap();
```

## Compatibility

Works on recent iOS, Mac, as well as Unix-family systems with a `pthread.h` that defines `pthread_getcpuclockid` (most modern Linux).
