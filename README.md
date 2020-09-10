# os_clock

Access various clocks (Per-Thread CPU Time, Monotomic) on Unix-family systems.

```rs
use std::io;
use os_clock::{self, Clock};

let clock = cpu_clock_for_current_thread();
let time = clock.get_time();

# Ok::<(), io::Error>(())

```

Notably, a clock for the CPU time of one thread can be accessed from another thread.
