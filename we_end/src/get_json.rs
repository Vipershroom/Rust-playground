use serde_json::json;
use std::fs::File;
use std::io::ErrorKind;
use crate::global::input;

pub fn check_prev() {
    let dir = String::new();
    loop {
            let j = File::open("settings.json");

            match j {
                Ok(file) => {file; break;},
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => {
                        create_json();
                    },
                    other_error => panic!("Their was an error {:?}", other_error)
                }
            };
        }
        println!("This is your H:Drive correct? (Y/n)");
        println!("\"H:\\2021_2022\"");
        let n = input();
        match n.to_lowercase().as_str() {
            "y" => write_json(&dir),
            _ => {
                println!("Please write your directory below");
                let dir = input();
                write_json(&dir)
            }
        }

}

fn create_json() {
    let f = File::create("settings.json");

    match f {
        Ok(file) => println!("settings.json was created succesfully"),
        Err(err) => println!("There was an error: {}", err),
    };
}

fn write_json(dir: &str) {
    println!("I was called")
}