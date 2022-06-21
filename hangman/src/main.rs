use hangman::input;
use std::{fs::{File, self}, io::Read};
use serde_json::{self, Value};

fn get_words(diff: String) {

    let diff = match diff.to_lowercase().trim(){
        "e" => "Easy",
        "m" => "Medium",
        "h" => "Hard",
        _ => panic!("Select a difficulty")
    };

    let mut f = File::open("words.json").unwrap_or_else(|error| {
        panic!("Error Reading words.json")
    });
    
    let contents = fs::read_to_string("words.json").unwrap_or_else(|error|{
        panic!("Error reading file")
    });

    let contents = serde_json::from_str::<Value>(&contents).unwrap();

    let length_of_word_list = &contents[diff]["Words"].to_string().len();

    println!("{}", &contents[diff]["Words"].to_string());

    println!("{}", length_of_word_list)

}

fn main() {
    println!("Welcome to my Hangman game!\n Please select your difficulty");
    println!("Easy(e), Medium(m), Hard(h)");
    let n = input();
    get_words(n);
}

fn game(diff: String) {
    
}