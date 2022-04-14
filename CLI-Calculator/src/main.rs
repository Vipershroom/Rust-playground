use std::io::Read;

fn input() -> String {
    let mut buffer = String::new();
    std::io::stdin()
    .read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn display_ui_1() {
        loop {
        println!("Options

>Addition(add)
>Subtraction(subtract)
>Multiplication(multi)
>Division(divide)
>Next
>Options");
        let request = input();
        match request.as_str() {
            "add" => addition(),
            "subtract" => subtraction(),
            "multi" => multiply(),
            "divide" => divide(),
            "next" => next(),
            _ => println!("Please input a valid option")
        }
    };
}

fn main() {
    println!("Welcome to my Calculator");
    display_ui_1()
}
