use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    let seconds = args
        .get(1)
        .expect("missing operand")
        .parse::<f64>()
        .expect("invalid time interval");

    thread::sleep(Duration::from_secs_f64(seconds));
}
