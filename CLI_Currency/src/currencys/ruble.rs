use crate::global::{num_input, input};

pub fn ruble(lang: &str) {
    match lang {
        "usd" => usd_to_ruble(),
        "yen" => yen_to_ruble(),
        "euro" => euro_to_ruble(),
        "real" => real_to_ruble(),
        _ => println!("This should never be hit")
    }
}

fn usd_to_ruble() {
    loop 
    {    println!("Please input your USD amount");
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

fn yen_to_ruble() {
    loop {
        println!("Please input your yen amount");
        let inp = num_input();
        let result = inp * 0.60;
        println!("¥{} yen is qual to ₽{:.2} ruble", inp, result);
        println!("Would you like to go again? (Y/n)");
            let yn = input();
            match yn.to_lowercase().as_str() {
                "y" => continue,
                _ => break
            }
    }
}

fn euro_to_ruble() {
    loop {
        println!("Please input your euro amount");
        let inp = num_input();
        let result = inp * 83.71;
        println!("€{} euro is equal to ₽{:.2} ruble", inp, result);
        println!("Would you like to go again? (Y/n)");
            let yn = input();
            match yn.to_lowercase().as_str() {
                "y" => continue,
                _ => break
            }
    }
}

fn real_to_ruble() {
    loop {
        println!("Please input your real amount");
        let inp = num_input();
        let result = inp * 16.12;
        println!("€{} real is equal to ₽{:.2} ruble", inp, result);
        println!("Would you like to go again? (Y/n)");
            let yn = input();
            match yn.to_lowercase().as_str() {
                "y" => continue,
                _ => break
            }
    }
}