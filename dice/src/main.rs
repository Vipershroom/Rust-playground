use rand::{thread_rng, Rng};

fn input() -> u32 {
    let mut buff = String::new();
    loop {
        buff.clear();
        std::io::stdin()
        .read_line(&mut buff)
        .unwrap();
        match buff.trim().parse() {
            Ok(input) => return input,
            Err(_) => println!("Please enter a positive integer:")
        }
    }
}

fn rand() -> u32 {
    thread_rng().gen_range(1..=6)
}
 
fn rolls(rollys: u32) -> Vec<u32> {
    let mut diceList: Vec<u32> = Vec::new();
    println!("");
    for i in 0..rollys {
        let currRoll = rand();
        diceList.push(currRoll);
        println!("{currRoll}")
    }
    println!("{:?}", diceList);
    diceList
}

fn sum(list: Vec<u32>) {
    let mut sam = 0;
    for i in list {
        sam += i;
    }
    println!("Your sum is {sam}")
}

fn main() {
    println!("Welcome to my dice game! \nPlease enter a number");
    let x = input();
    let lisp = rolls(x);
    sum(lisp);
}