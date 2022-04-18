use crate::global::{int_input, input};

pub fn display_ui_2() {
    loop {
        println!("> Square
> Cube
> Square root
> Abs
> Log");
        let request = input();
        match request.to_lowercase().as_str() {
            // "square" => cube(),
            // "cube" => square(),
            // "abs" => abs(),
            // "log" => log(),
            _ => println!("Please enter a valid option")
        }
    }
}