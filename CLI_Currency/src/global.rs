pub fn input() -> String  {
    let mut buff = String::new();

    std::io::stdin().
    read_line(&mut buff).unwrap();

    buff.trim().to_string()
}