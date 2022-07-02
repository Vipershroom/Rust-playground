use hangman::*;

fn main() {
    println!("Welcome to Hangamn!");
    let word = get_random_word();
    let word_arr: Vec<char> = word.chars().collect();
    let state: usize = 0;
    let dead = false;
    let mut guessed_words: Vec<String> = vec![];

    // loop {
        // Check if the player is dead
        if dead == true {
            // break;
        };
        render_game(state, word.clone(), guessed_words.clone());
        let letter = input();
        for i in word_arr {
            if letter == i.to_string() {
                
            }
        }

        // Take input for a letter
        // Update the game state as to whether it is a letter or not
    // }
}