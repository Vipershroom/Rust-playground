#![allow(non_snake_case)]
use CLI_Test::input;

fn main() {
    println!("Hello, world!");
    let n = input();
    println!("{n}");
    let splitted: Vec<&str> = n.split(" ").collect();
    println!("{:?}",splitted)
}

