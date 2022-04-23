use crate::global::{num_input, input};

pub fn euro(lang: &str) {
    match lang {
        "usd" => usd_to_euro(),
        "yen" => yen_to_euro(),
        "ruble" => ruble_to_euro(),
        "real" => real_to_euro(),
        _ => println!("This should never be hit")
    }
}

fn usd_to_euro() {

}

fn yen_to_euro() {

}

fn ruble_to_euro() {

}

fn real_to_euro() {
    
}