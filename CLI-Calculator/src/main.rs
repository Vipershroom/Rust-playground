use crate::global::input;


mod arithmatic;
mod global;

fn display_ui_1() {
        loop {
        println!("
>Addition(add)
>Subtraction(subtract)
>Multiplication(multi)
>Division(divide)
>Interactive mode(Not finished)
>Next
>Options
>Exit"
);
        let request = input();
        match request.as_str() {
            "add" => arithmatic::addition(),
            // "subtract" => arithmatic::subtraction(),
            // "multi" => arithmatic::multiply(),
            // "divide" => arithmatic::divide(),
            // "next" => arithmatic::next(),
            _ => println!("Please input a valid option")
        }
    };
}

fn main() {
    println!("Welcome to my Calculator");
    display_ui_1()
}
