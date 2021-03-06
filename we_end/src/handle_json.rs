use serde_json::Value;
use std::fs::File;
use std::fs;
use std::io::{ErrorKind, Read};
use crate::global::input;


// Simple check needed to check if there 
// is a settings.json
// if a settings.json is found it skips
// if not it will call the create_json function

pub fn check_prev() {
    loop {
            let j = File::open("settings.json");

            match j {
                Ok(_file) => { 
                    println!("settings.json found!");
                    break;},
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => {
                        create_json();
                    },
                    _other_error => {
                        println!("Failure to check whether settings.json exists.\n Press enter to quit");
                        input();
                        panic!("Failure to check whether settings.json exists.\n Press enter to quit")
                    }
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
        Ok(_file) => println!("settings.json was created succesfully"),
        Err(_err) => {
            println!("There was a problem creating settings.json");
            println!("Press enter to quit");
            input();
            panic!("Program exited due failure to create settings.json")
        },
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
    match fs::write("settings.json", write_val) {
        Ok(write) => write,
        Err(_) => {
            println!("Error writing to file\nPress enter to exit");
            panic!("Error writing to file")
        }
    }
}



pub fn read_json() -> String {
    let mut f = match File::open("settings.json") {
        Ok(read) => read,
        Err(_) => {
            println!("Failed to read settings.json\nPress enter to quit");
            input();
            panic!("Failed to read settings.json")
        }
    };

    let mut json_contents = String::new();
    f.read_to_string(&mut json_contents).expect("Failed to read file");
    let contents = serde_json::from_str::<Value>(&json_contents).unwrap();
    let dir_string = contents["directory"].as_str().unwrap();
    dir_string.to_string()
}
