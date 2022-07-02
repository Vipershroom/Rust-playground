use rand::Rng;

pub fn input() -> String {
    let mut buff = String::new();

    std::io::stdin().read_line(&mut buff).unwrap();

    buff.trim().to_string()
}

pub fn get_random_word() -> String {
    let word_bank = [
    "houseplan", "present", "Taylor",
    "Straight", "People", "deviation",
    "matrix", "vector", "paramater"];
    let rand_num = rand::thread_rng().gen_range(0..word_bank.len() - 1);
    word_bank[rand_num].to_string()
}

fn gen_underscores(word: String, guessed_words: Vec<String>) {
    let word_arr: Vec<char> = word.chars().collect();
    let mut cond = false;
    println!("{}", word);
    for i in word_arr {
        for n in guessed_words.clone() {
            if i.to_string() == n {
                print!(" {i} ");
                cond = true;
            } 
        }
        if cond == true {
            cond = false;
            
        } else {
            print!(" _ ");
        }
    }
    println!("")
}

fn hangman_state(state: usize) -> String {
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

  let mut hangman: Vec<String> = m.iter().map(|&s| s.into()).collect(); 
  let hangman = hangman.remove(state);
  hangman
}

pub fn render_game(state: usize, word: String, guessed_words: Vec<String>) {
    println!("{}",hangman_state(state));
    gen_underscores(word, guessed_words);
}