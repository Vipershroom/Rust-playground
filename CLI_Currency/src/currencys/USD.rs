use crate::global::{num_input,input};

pub fn usd(lang: &str) {
    match lang {
        "yen" => yen_to_usd(),
        "ruble" => ruble_to_usd(),
        "euro" => euro_to_usd(),
        "real" => real_to_usd(),
        _ => println!("This should never hit")
    }
}

fn yen_to_usd() {
    loop {
        println!("Please input your yen amount");
        let inp = num_input();
        let result = inp * 0.0078;
        println!("{} yen is equal to ${:.2} USD",inp, result);
        println!("Would you like to go again? (Y/n)");
        let yn = input();
        match yn.to_lowercase().as_str() {
            "y" => continue,
            _ => break
        }
    }
}

fn ruble_to_usd() {
    loop 
    {    println!("Please input your ruble amount");
        let inp = num_input();
        let result = inp * 0.013;
        println!("{} ruble is equal to ${:.2} USD", inp, result);
        println!("Would you like to go again? (Y/n)");
            let yn = input();
            match yn.to_lowercase().as_str() {
                "y" => continue,
                _ => break
            }
        }
}

fn euro_to_usd() {
    loop 
    {    println!("Please input your euro amount");
        let inp = num_input();
        let result = inp * 1.08;
        println!("{} Euro is equal to ${:.2} USD", inp, result);
        println!("Would you like to go again? (Y/n)");
            let yn = input();
            match yn.to_lowercase().as_str() {
                "y" => continue,
                _ => break
            }
    }
}

fn real_to_usd() {
    loop {
        println!("Please input your real amount");
        let inp = num_input();
        let result = inp * 0.21;
        println!("{} Real is equal to ${:.2} USD", inp, result)
    }
}