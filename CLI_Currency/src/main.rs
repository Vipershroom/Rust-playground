

use std::{collections::HashMap};

use crate::global::{input, num_input};

mod global;
mod currencys;

fn default_currency() -> String{
    loop {
        println!("Please enter your default currency");
        let m = vec!["USD", "Yen", "Ruble", "Euro", "Real"];
        for i in &m {
            println!(">{i}");
        }
        let cur = input();
        match cur.to_lowercase().as_str() {
            "usd" => return m[0].to_string(),
            "yen" => return m[1].to_string(),
            "ruble" => return m[2].to_string(),
            "euro" => return m[3].to_string(),
            "real" => return m[4].to_string(),
            _ => {
                println!("\nPlease enter a valid currency\n");
                continue
            }
        }
    }
}

fn display_screen() {
    let mut default = default_currency();
    let langlist = vec!["USD", "Yen", "Ruble", "Euro", "Real"];
    loop {
        println!("\nYour default language is {}", &default);
        for i in &langlist {
            if i.to_lowercase() != default.to_lowercase() {
                println!(">{}", i)
            } 
        }

        let m = input();
        match m.to_lowercase().as_str() {
            "usd" => currencys::usd(&default.to_lowercase()),
            "yen" => currencys::yen(),
            _ => println!("\n Please enter a valid value")
        }
    }
}


fn main() {
    println!("Welcome to my CLI Currency translator");
    display_screen()
}
