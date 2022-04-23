use crate::global::{num_input, input};

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
    loop {
        println!("Please input your USD amount");
        let inp = num_input();
        let result = inp * 128.47;
        println!("${} USD is ¥{} Yen", inp, result);
        println!("Would you like to go again? (Y/n)");
        let yn = input();
        match yn.to_lowercase().as_str() {
            "y" => continue,
            _ => break
        }
    }
}

fn ruble_to_yen() {

}

fn euro_to_yen() {

}

fn real_to_yen() {

}