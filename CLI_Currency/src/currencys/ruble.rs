use crate::global::{num_input, input};

pub fn ruble(lang: &str) {
    match lang {
        "usd" => usd_to_ruble(),
        "yen" => yen_to_usd(),
        "euro" => euro_to_ruble(),
        "real" => real_to_ruble(),
        _ => println!("This should never be hit")
    }
}

fn usd_to_ruble() {
    loop 
    {    println!("Please input your ruble amount");
        let inp = num_input();
        let result = inp * 0.013;
        println!("${} USD is equal to ₽{:.2} ruble", inp, result);
        println!("Would you like to go again? (Y/n)");
            let yn = input();
            match yn.to_lowercase().as_str() {
                "y" => continue,
                _ => break
            }
        }
}

fn yen_to_usd() {

}

fn euro_to_ruble() {

}

fn real_to_ruble() {

}