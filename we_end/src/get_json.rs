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

}

fn create_json() {
    input();
}

fn write_json() {}