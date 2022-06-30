use rand::Rng;

pub fn input() -> String {
    let mut buff = String::new();

    std::io::stdin().read_line(&mut buff).unwrap();

    buff.trim().to_string()
}

pub fn get_random_word() -> String {
    let word_bank = ["
    houseplan", "present", "Taylor",
    "Straight", "People", "deviation",
    "matrix", "vector", "paramater"];
    let rand_num = rand::thread_rng().gen_range(0..word_bank.len() - 1);
    word_bank[rand_num].to_string()
}

pub fn hangman_state() -> Vec<String> {
    let m = ["+---+
    |   |
        |
        |
        |
        |
  ========="
    ,"+---+
    |   |
    O   |
        |
        |
        |
  =========",
  " +---+
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
      |   |
      O   |
     /|\\  |
          |
          |
    =========",
    "+---+
    |   |
    O   |
   /|\\  |
   /    |
        |
  =========",
  "+---+
  |   |
  O   |
 /|\\  |
 / \\  |
      |
========="];

  let hangman: Vec<String> = m.iter().map(|&s| s.into()).collect(); 
  hangman
}