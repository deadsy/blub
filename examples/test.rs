use std::time::Instant;

use blub::lut::pow2;

const ITERS: i32 = 800000;

fn main() {
    let start = Instant::now();

    for _i in 0..ITERS {
        pow2(1.1);
        pow2(2.2);
        pow2(7.3);
        pow2(-8.8);
        pow2(17.3);
    }

    let stop = Instant::now();
    let elapsed = stop.duration_since(start).subsec_nanos();
    let op_speed = (elapsed as f64) / ((ITERS as f64) * 5.0);

    println!("{}", op_speed);
}
