use std::time::{Duration, Instant};

pub fn loops() {
    println!("start while loop");
    while_loop();
    println!("end while loop; start loop");
    lets_use_loop();
    println!("end loop; start nested loops");
    nested_loops();
}

fn while_loop() {
    let mut count = 0;
    let one_second = Duration::new(1, 0);
    let start = Instant::now(); // accesses time from the system's clock

    while (Instant::now() - start) < one_second {
        count += 1;
    }
    println!("count: {}", count);
}

fn lets_use_loop() {
    let mut count = 0;
    // `loop` is often used when implementing a long-running server
    loop {
        if count > 10 {
            break;
        }
        count += 1;
    }
    println!("final count: {}", count)
}

fn nested_loops() {
    let start = Instant::now();
    // `'outer` = loop label
    'outer: for x in 0.. {
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 50000 {
                    let duration = match Instant::now().checked_duration_since(start) {
                        Some(d) => d,
                        None => panic!(),
                    };
                    println!(
                        "made it to 50,000 in... {} nanoseconds // {} microseconds // {} milliseconds",
                        duration.as_nanos(),
                        duration.as_micros(),
                        duration.as_millis()
                    );
                    break 'outer;
                }
            }
        }
    }
}
