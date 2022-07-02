use hangman_new::*;
use std::collections::HashSet;

fn main() {
    println!("Welcome to hangman!");

    let mut state = 0;
    let word = get_random_word();
    let word_arr: Vec<char> = word.chars().collect();
    let mut guessed_wrong_words: Vec<String> = Vec::new();
    let mut guessed_right_words: HashSet<String> = HashSet::new();

    render_initial_game(&state, &word);

    loop {
        if state > 7 {
            break;
        }

        let guess = input();

        if compare_two_list(&guess, &word_arr) {
            guessed_right_words.insert(guess.clone());
        } else {
            guessed_wrong_words.push(guess.clone());
                state += 1
        }

        render_game(&state, &word, &guessed_right_words)
    }
    // Update the game whenever the user gets a letter wrong or right.
    
}
