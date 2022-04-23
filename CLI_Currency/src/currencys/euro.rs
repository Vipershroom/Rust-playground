use crate::global::{num_input, input};

pub fn euro(lang: &str) {
    match lang {
        "usd" => usd_to_euro(),
        "yen" => yen_to_euro(),
        "ruble" => ruble_to_euro(),
        "real" => real_to_euro(),
        "euro" => println!("Ok, funny man"),
        _ => println!("This should never be hit")
    }
}

fn usd_to_euro() {
    loop {
        println!("Please input your usd amount");
        let inp = num_input();
        let result = inp * 0.93;
        println!("${} USD is equal to €{:.2} euros", inp, result);
        println!("Would you like to go again? (Y/n)");
            let yn = input();
            match yn.to_lowercase().as_str() {
                "y" => continue,
                _ => break
        }
    }
}

fn yen_to_euro() {
    loop {
        println!("Please input your yen amount");
        let inp = num_input();
        let result = inp * 0.0072;
        println!("¥{} yen is equal to €{:.2} euros", inp, result);
        println!("Would you like to go again? (Y/n)");
            let yn = input();
            match yn.to_lowercase().as_str() {
                "y" => continue,
                _ => break
        }
    }
}

fn ruble_to_euro() {
    loop {
        println!("Please input your ruble amount");
        let inp = num_input();
        let result = inp * 0.012;
        println!("₽{} ruble is equal to €{:.2} euros", inp, result);
        println!("Would you like to go again? (Y/n)");
            let yn = input();
            match yn.to_lowercase().as_str() {
                "y" => continue,
                _ => break
        }
    }
}

fn real_to_euro() {
    loop {
        println!("Please input your real amount");
        let inp = num_input();
        let result = inp * 0.19;
        println!("R${} real is equal to €{:.2} euros", inp, result);
        println!("Would you like to go again? (Y/n)");
            let yn = input();
            match yn.to_lowercase().as_str() {
                "y" => continue,
                _ => break
        }
    }
}