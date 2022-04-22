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

        let intBuff: i32 = match buff.trim().parse()  {
            Ok(input) => input,
            Err(_) => {println!("Please enter a valid value");
                      continue}
        };
        return intBuff as f32;
    }
}