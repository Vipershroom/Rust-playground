use crate::global::num_input;

pub fn usd(lang: &str) {
    let curLang = vec!["Yen", "Ruble", "Euro", "Real"];
    match lang {
        "yen" => yen_to_usd(),
        "ruble" => ruble_to_usd(),
        "euro" => euro_to_usd(),
        // "real" => real_to_usd(),
        _ => println!("This should never hit")
    }
}

fn yen_to_usd() {
    println!("Please input your yen amount");
    let inp = num_input();
    let result = inp * 0.0078;
    println!("{} yen is equal to ${:.2} USD",inp, result)
}

fn ruble_to_usd() {
    println!("Please input your ruble amount");
    let inp = num_input();
    let result = inp * 0.013;
    println!("{} ruble is equal to ${:.2} USD", inp, result)
}

fn euro_to_usd() {
    println!("Please input your euro amount");
    let inp = num_input();
    let result = inp * 1.08;
    println!("{} Euro is equal to ${:.2}", inp, result)
}