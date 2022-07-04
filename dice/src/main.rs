use std::io::stdin;
use rand::Rng;

fn main() {
    println!("Please enter a number");
    let n = input();
    let dice_vec = process_rolls(n);
    println!("Your rolls are: {:?}", dice_vec);
    println!("The sum is: {}", sum_rolls(dice_vec))
}

fn input() -> u32 {
    let mut buf = String::new();

    stdin().read_line(&mut buf).unwrap();

    match buf.trim().parse() {
        Ok(input) => return input,
        Err(_) => panic!("Please enter a positive integer")
    }
}

fn roll_dice() -> u32 {
    rand::thread_rng().gen_range(1..6)
}

fn process_rolls(n: u32) -> Vec<u32> {
    (0..n).map(|_| roll_dice()).collect()
}

fn sum_rolls(n: Vec<u32>) -> u32{
    n.iter().sum()
}