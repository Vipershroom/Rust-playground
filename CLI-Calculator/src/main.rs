use crate::global::input;

mod display_2;
mod arithmatic;
mod global;

fn display_ui_1() {
        loop {
        println!("> Addition(add)
> Subtraction(subtract)
> Multiplication(multi)
> Division(divide)
> Next
> Interactive mode(Not finished)
> Options
> Exit"
);
        let request = input();
        match request.to_lowercase().as_str() {
            "add" => arithmatic::addition(),
            "subtract" => arithmatic::subtraction(),
            "multi" => arithmatic::multiply(),
            "divide" => arithmatic::divide(),
            "next" => display_2::display_ui_2(),
            "exit" => {println!("Exiting....");
                      return}
            _ => println!("Please input a valid option")
        }
    };
}

fn main() {
    println!("Welcome to my Calculator");
    display_ui_1()
}
