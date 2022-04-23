use crate::global::{num_input, input};

pub fn yen(lang: &str) {
    match lang {
        "usd" => usd_to_yen(),
        "ruble" => ruble_to_yen(),
        "euro" => euro_to_yen(),
        "real" => real_to_yen(),
        "yen" => println!("Ok, funny man"),
        _ => println!("This should never be hit")
    }
}

fn usd_to_yen() {
    loop {
        println!("Please input your USD amount");
        let inp = num_input();
        let result = inp * 128.47;
        println!("${} USD is ¥{:.2} yen", inp, result);
        println!("Would you like to go again? (Y/n)");
        let yn = input();
        match yn.to_lowercase().as_str() {
            "y" => continue,
            _ => break
        }
    }
}

fn ruble_to_yen() {
    loop {
        println!("Please input your ruble amount");
        let inp = num_input();
        let result = inp * 1.66;
        println!("₽{} ruble is ¥{:.2} yen", inp, result);
        println!("Would you like to go again? (Y/n)");
        let yn = input();
        match yn.to_lowercase().as_str() {
            "y" => continue,
            _ => break
        }
    }
}

fn euro_to_yen() {
    loop {
        println!("Please input your euro amount");
        let inp = num_input();
        let result = inp * 138.84;
        println!("€{} euro is ¥{:2} yen", inp, result);
        println!("Would you like to go again? (Y/n)");
        let yn = input();
        match yn.to_lowercase().as_str() {
            "y" => continue,
            _ => break
        }
    }
}

fn real_to_yen() {
    loop {
        println!("Please input your real amount");
        let inp = num_input();
        let result = inp * 26.77;
        println!("R${} real is ¥{:.2} yen", inp, result);
        println!("Would you like to go again? (Y/n)");
        let yn = input();
        match yn.to_lowercase().as_str() {
            "y" => continue,
            _ => break
        }
    }
}