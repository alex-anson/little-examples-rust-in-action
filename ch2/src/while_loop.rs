use std::time::{Duration, Instant};

pub fn while_loop() {
    let mut count = 0;
    let one_second = Duration::new(1, 0);
    let start = Instant::now(); // accesses time from the system's clock

    while (Instant::now() - start) < one_second {
        count += 1;
    }
    println!("count: {}", count);
}
