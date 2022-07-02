use hangman_new::*;

fn main() {
    println!("Welcome to hangman!");

    let mut state = 0;
    let word = get_random_word();
    let word_arr: Vec<char> = word.chars().collect();
    let mut guessed_wrong_words: Vec<String> = Vec::new();
    let mut guessed_right_words: Vec<String> = Vec::new();

    render_initial_game(&state, &word);

    loop {
        if state > 7 {
            break;
        }

        
        let guess = input();
        
        if compare_two_list(&guess, &word_arr) {
            guessed_right_words.push(guess.clone())
        } else {
            guessed_wrong_words.push(guess.clone());
                println!("HI");
                state += 1
        }

        render_game(&state, &word, guess, &guessed_right_words, &guessed_wrong_words)
    }
    // Update the game whenever the user gets a letter wrong or right.
    
}
