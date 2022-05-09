use std::{thread, time};

fn main() {
    thread::sleep(time::Duration::from_millis(1000));
    println!("Hello, world!");
}
