use crate::global::{input, num_input};

pub fn real(lang: &str) {
    match lang {
        "usd" => usd_to_real(),
        "yen" => yen_to_real(),
        "ruble" => ruble_to_real(),
        "euro" => euro_to_real(),
        _ => println!("This should never be hit"),
    }
}

fn usd_to_real() {

}

fn yen_to_real() {

}

fn ruble_to_real() {

}

fn euro_to_real() {
    
}