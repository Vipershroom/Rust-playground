mod global;
mod currencys;

fn default_currency() {
    println!("Please enter your default currency")
}

fn display_screen() {
    println!("
Default: USD
>Japanese(Yen)
>Russian(Ruble)
>EU(Euro)
>Brazilian (Real)
>Options");

    let m = input();
    match m.to_lowercase().as_str() {
        "yen" => yen(),
    }
}


fn main() {
    println!("Welcome to my CLI Currency translator");
    display_screen()
}
