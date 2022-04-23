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
    loop {
        println!("Please input your usd amount");
        let inp = num_input();
        let result = inp * 4.80;
        println!("${} USD is equal to R${:.2} real", inp, result);
        println!("Would you like to go again? (Y/n)");
            let yn = input();
            match yn.to_lowercase().as_str() {
                "y" => continue,
                _ => break
        }
    }
}

fn yen_to_real() {
    loop {
        println!("Please input your yen amount");
        let inp = num_input();
        let result = inp * 0.037;
        println!("¥{} yen is equal to R${:.2} real", inp, result);
        println!("Would you like to go again? (Y/n)");
            let yn = input();
            match yn.to_lowercase().as_str() {
                "y" => continue,
                _ => break
        }
    }
}

fn ruble_to_real() {
    loop {
        println!("Please input your ruble amount");
        let inp = num_input();
        let result = inp * 0.062;
        println!("₽{} ruble is equal to R${:.2} real", inp, result);
        println!("Would you like to go again? (Y/n)");
            let yn = input();
            match yn.to_lowercase().as_str() {
                "y" => continue,
                _ => break
        }
    }
}

fn euro_to_real() {
    loop {
        println!("Please input your euro amount");
        let inp = num_input();
        let result = inp * 5.19;
        println!("€{} euro is equal to R${:.2} real", inp, result);
        println!("Would you like to go again? (Y/n)");
            let yn = input();
            match yn.to_lowercase().as_str() {
                "y" => continue,
                _ => break
        }
    }
}