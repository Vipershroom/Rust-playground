use serde_json::json;
use std::fs::File;
use std::fs;
use std::io::{ErrorKind, Write};
use crate::global::input;


// Simple check needed to check if there 
// is a settings.json
// if a settings.json is found it skips
// if not it will call the create_json function

pub fn check_prev() {
    loop {
            let j = File::open("settings.json");

            match j {
                Ok(file) => { 
                    println!("settings.json found!");
                    break;},
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => {
                        create_json();
                    },
                    other_error => panic!("Their was an error {:?}", other_error)
                }
            };
        }
        

}

// Handles the creation of the json file
// In the case where a settings.json is not present
// This code will run to create one.

fn create_json() {
    let f = File::create("settings.json");
    let dir = String::from("H:\\\\2021_2022\\\\");

    match f {
        Ok(file) => println!("settings.json was created succesfully"),
        Err(err) => println!("There was an error: {}", err),
    };
    println!("This is your H:Drive correct? (Y/n)");
    println!("\"H:\\2021_2022\"");
    let n = input();
        match n.to_lowercase().as_str() {
            "y" => write_json(&dir),
            _ => {
                println!("Please write your directory below");
                println!("If you are on windows please add another \\ to your directorys");
                println!("Example: H:\\\\2021_2022\\\\");
                let dir = input();
                write_json(&dir)
            }
        }
}

// Needed to on creation of a settings.json
// to fill in the directory for the program
// to know where to place the files.

fn write_json(dir: &str) {
    let write_val = format!(r#"{{
    "directory": "{}"
}}
    "#, dir);
    fs::write("settings.json", write_val).expect("Unable to write to file")
}

fn read_json() {}