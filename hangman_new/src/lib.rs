use std::collections::HashSet;

use rand::Rng;

pub fn input() -> String {
    let mut buff = String::new();

    std::io::stdin().read_line(&mut buff).unwrap();

    buff.trim().to_string()
}

pub fn get_random_word() -> String {
    let word_bank = [
    "houseplan", "present", "taylor",
    "straight", "people", "deviation",
    "matrix", "vector", "paramater"];
    let rand_num = rand::thread_rng().gen_range(0..word_bank.len() - 1);
    word_bank[rand_num].to_string()
}

pub fn compare_two_list(word: &str, vecc: &Vec<char>) -> bool{
    for i in vecc {
        if word == i.to_string() {
            return true
        }
    }
    false
}

fn hangman_state(state: &usize) -> &'static str {
    let m = [
    "+---+
    |   |
        |
        |
        |
        |
  =========",
    "+---+
    |   |
    O   |
        |
        |
        |
  =========",
    "+---+
    |   |
    O   |
    |   |
        |
        |
=========",
    "+---+
    |   |
    O   |
   /|   |
        |
        |
    =========",
    "+---+
    |    |
    O    |
   /|\\   |
         |
         |
    =========",
    "+---+
    |    |
    O    |
   /|\\   |
   /     |
         |
  =========",
    "+---+
    |    |
    O    |
   /|\\   |
   / \\   |
         |
========="];

    m[*state]
}



fn gen_underscores(word: &str, right_guess: &HashSet<String>) {
    let mut cond = false;
    for i in word.chars() {
        for j in right_guess {
            if &i.to_string() == j {
                print!("{i} ");
                cond = true
            }
        }
        if cond == false {
            print!(" _ ")
        } else {
            cond = false
        }
    }
    println!()
}

pub fn render_initial_game(state: &usize, word: &str) {
    println!("{}", hangman_state(&state));
    for _i in 0..word.len() {
        print!(" _ ");
    }
    println!()
}

pub fn render_game(state: &usize, word: &str, right_guess: &HashSet<String>) {
    println!("{}", hangman_state(&state));
    gen_underscores(word, right_guess)
}

