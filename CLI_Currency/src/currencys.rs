use crate::global::num_input;

pub fn usd(lang: &str) {
    let curLang = vec!["Yen", "Ruble", "Euro", "Real"];
    match lang {
        "yen" => yen_to_usd(),
        _ => println!("This should never hit")
    }
}

fn yen_to_usd() {
    println!("Please input your yen amount");
    let inp = num_input();
    let result = inp * 0.0078;
    println!("{} yen is equal to {} usd",inp, result)
}

pub fn yen() {
    
}