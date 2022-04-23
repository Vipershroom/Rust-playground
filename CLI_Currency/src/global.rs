pub fn input() -> String  {
    let mut buff = String::new();

    std::io::stdin().
    read_line(&mut buff).unwrap();

    buff.trim().to_string()
}

pub fn num_input() -> f32 {
    loop{
        let mut buff = String::new();

        std::io::stdin().
        read_line(&mut buff).unwrap();

        match buff.trim().parse() {
            Ok(input) => return input,
            Err(_) =>  {
                println!("Please input a valid value");
                println!("Hint! If you entered a hole number add .0 to the end")
            }
        }
    }
}