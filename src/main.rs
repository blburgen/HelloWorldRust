// imports random number generation methods
extern crate rand;
use rand::Rng;
// core file reference handle -- create, read, write, and query metadata for files
use std::fs::File;
// core input/output trates
use std::io::prelude::*;
// imports the standard input/output module
use std::io;
// constant value
const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter {
    character: char,
    revealed: bool,
}

enum GameProgress {
    InProgress,
    Won,
    Lost,
}

fn main() {
    // mutable variable
    let mut turns_left = ALLOWED_ATTEMPTS;
    // non-mutable variable
    let selected_word = select_word();
    // mutable variable
    let mut letters = create_letters(&selected_word);

    //main loop
    loop {
        // println! prints to the console with a return
        println!("You have {} turns left.", turns_left);
        display_progress(&letters);

        // print! prints to the console without a return
        println!("Please enter a letter to guess: ");
        let user_char = read_user_input_character();

        // Exit if user enters an asterisk '*'
        if user_char == '*' {
            break;
        }

        // Updates the revealed state of each letter
        let mut at_least_one_revealed = false;
        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }

        // If they guessed incorrectly loose a turn
        if !at_least_one_revealed {
            turns_left -= 1;
        }

        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("\nCongrats! You won! ☺");
                break;
            }
            GameProgress::Lost => {
                println!("\nYou lost! ☹");
                break;
            }
        }
    }
    // println! prints to the console
    println!("Selected word was {}", selected_word);
}

fn select_word() -> String {
    let mut file = File::open("words.txt").expect("Could not open file!");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("An error occured while reading the file!");

    let available_words: Vec<&str> = file_contents.split(',').collect();

    // Select word at random 
    let random_index = rand::thread_rng().gen_range(0, available_words.len());

    return String::from(available_words[random_index]);
}

fn create_letters(word: &String) -> Vec<Letter> {
    // Create empty vector
    let mut letters: Vec<Letter> = Vec::new();

    // Wrap each character in a Letter struct
    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false,
        });
    }

    return letters;
}

fn display_progress(letters: &Vec<Letter>) {
    let mut display_string = String::from("Progress:");

    // Display appropriate character (letter or _) for each letter
    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }

        display_string.push(' ');
    }

    println!("{}", display_string);
}

fn check_progress(turns_left: u8, letters: &Vec<Letter>) -> GameProgress {
    // Determine if all letters have been revealed or if you have lost
    let mut all_revealed = true;
    for letter in letters {
        if !letter.revealed {
            all_revealed = false;
        }
    }

    if all_revealed {
        return GameProgress::Won;
    }

    if turns_left > 0 {
        return GameProgress::InProgress;
    }

    return GameProgress::Lost;
}

fn read_user_input_character() -> char {
    // mutable string variable
    let mut user_input = String::new();

    // Get user input
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => match user_input.chars().next() {
            Some(c) => {
                return c;
            }
            None => {
                return '*';
            }
        },
        Err(_) => {
            return '*';
        }
    }
}
