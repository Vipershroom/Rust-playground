use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn input() -> u32 {
    let mut buffer = String::new();
    loop {
        buffer.clear();
        io::stdin()
        .read_line(&mut buffer)
        .unwrap();

        match buffer.trim().parse() {
            Ok(input) => return input,
            Err(_) => println!("Please input a positive integer")
        }
    }
}

fn rand_num() -> u32 {
    thread_rng().gen_range(1..=100)
}

fn guess() {
    let secret_num = rand_num();
    let mut guess_amt = 1;
    loop {
        println!("Number of guesses: {}", guess_amt);
        println!("Please guess a number:");
        let inp = input();
        guess_amt += 1;
        println!("");
        match inp.cmp(&secret_num) {
            Ordering::Greater => println!("Guess lower"),
            Ordering::Less => println!("Guess higher"),
            Ordering::Equal => {
                win(guess_amt, secret_num);
                break;
            },
            _ => println!("Please guess a positive integer")
        }
    }
}

fn win(guess_amt: u32, secret: u32) {
    println!("Congrats the number was {}", secret);
    println!("You did it in {} guesses", guess_amt)
}

fn main() {
    println!("Welcome to my guesssing game");
    guess();
}