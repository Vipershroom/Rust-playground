use hangman::input;
use std::{fs::File, io::Read};
use serde_json::{self, Value};

fn get_words(diff: String) {
    let mut f = File::open("words.json").unwrap_or_else(|error| {
        panic!("Error Reading words.json")
    });
    let mut json_contents = String::new();
    f.read_to_string(&mut json_contents).expect("Failed to read file");
    let contents = serde_json::from_str::<Value>(&json_contents).unwrap();
    

    let diff = match diff.to_lowercase().trim(){
        "e" => "Easy",
        "m" => "Medium",
        "h" => "Hard",
        _ => panic!("Select a difficulty")
    };

    let dir_string = contents["Easy"].as_str().unwrap();
    println!()

}

fn main() {
    println!("Welcome to my Hangman game!\n Please select your difficulty");
    println!("Easy(e), Medium(m), Hard(h)");
    let n = input();
    get_words(n);
}

fn game(diff: String) {
    
}