# os_clock

Access various operating system clocks (such as per-thread CPU Time, system clock, monotomic, etc) on Unix-family systems.

```rs
use std::io;
use os_clock::{self, Clock};

let clock = cpu_clock_for_current_thread();
let time = clock.get_time();

# Ok::<(), io::Error>(())

```

Notably, a clock for the CPU time of one thread can be accessed from another thread.


## Compatibility

Works on recent iOS, Mac, as well as Unix-family systems with a `pthread.h` that defines `pthread_getcpuclockid` (most modern Linux).
