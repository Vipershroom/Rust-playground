use crate::global::num_input;

pub fn yen(lang: &str) {
    match lang {
        "usd" => usd_to_yen(),
        "ruble" => ruble_to_yen(),
        "euro" => euro_to_yen(),
        "real" => real_to_yen(),
        _ => println!("This should never be hit")
    }
}

fn usd_to_yen() {

}

fn ruble_to_yen() {

}

fn euro_to_yen() {

}

fn real_to_yen() {

}