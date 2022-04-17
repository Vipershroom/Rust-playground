use std::thread;

use crate::global::{int_input, input};

pub fn addition() {
    println!("Please input a number!");
    let num1 = int_input();
    println!("Please input another number");
    let num2 = int_input();
    println!("Your result is {}", num1 + num2);
    println!("Enter a key to go back");
    input();
}

