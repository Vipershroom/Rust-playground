pub fn input() -> String {
    let mut buffer = String::new();
    std::io::stdin()
    .read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

pub fn int_input() -> i32 {
    loop {
        let mut buffer = String::new();
        std::io::stdin()
        .read_line(&mut buffer).unwrap();
        match buffer.trim().parse() {
            Ok(input) => return input,
            Err(_) => {println!("Please enter a valid number");},
        }
    }
}