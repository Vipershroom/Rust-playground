use crate::global::input;

mod global;
mod currencys;

fn default_currency() {
    println!("Please enter your default currency")
}

fn display_screen() {
    let mut default = "USD";
    let m = ["USD", "Japanese", "Russian", "EU", "Brazillian"];
    loop {
        println!("
Default: {}
>Japanese(Yen)
>Russian(Ruble)
>EU(Euro)
>Brazilian (Real)
>Options
>Exit", default);

        let m = input();
        match m.to_lowercase().as_str() {
            "yen" => currencys::yen(),
            _ => println!("Please enter a valid value")
        }
    }
}


fn main() {
    println!("Welcome to my CLI Currency translator");
    display_screen()
}
