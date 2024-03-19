extern crate rand;
use rand::Rng;
use std::fs;
use std::io;

struct Letter {
    letter : char,
    found : bool
}

fn main() {
    let word = choose_word();
    let mut lives: i32 = 5;
    let mut all_found: bool;
    let mut user_guesses: Vec<char> = Vec::new();
    let mut game_state: Vec<Letter> = Vec::new();
    println!("Welcome to Hangman!");
    for i in word.chars() {
        let letter : Letter = Letter {
            letter: i,
            found: false
        };
        game_state.push(letter);
    }
    while lives > 0 {
        if lives > 1 {
            println!("You have {} lives!", lives);
        }
        else
        {
            println!("You have {} life!", lives);
        }
        print_game(&game_state);
        println!("Please guess a letter!");
        let mut guess = get_guess();
        while !guess.is_alphabetic() || user_guesses.contains(&guess) {
            println!("Please enter a valid letter!");
            if !guess.is_alphabetic() {
                println!("Please enter a valid letter!");
            }
            if user_guesses.contains(&guess) {
                println!("You already guessed this letter, please enter a new letter!");
            }
            guess = get_guess();
        }
        user_guesses.push(guess);
        if !word.contains(guess) {
            all_found = false;
            lives -= 1;
        }
        else {
            all_found = true;
            for letter in &mut game_state {
                if letter.letter == guess {
                    letter.found = true;
                }
                else {
                    if !letter.found {
                        all_found = false;
                    }
                }
            }
        }
        if all_found {
            print_game(&game_state);
            println!("Congratulations, you won!");
            return;
        }
    }
    println!("Game over, better luck next time!");
}

fn print_game(game_state: &Vec<Letter>) {
    for i in game_state {
        if i.found {
            print!("{} ", i.letter);
        }
        else {
            print!("_ ");
        }
    }
    println!();
}

fn get_guess() -> char {
    let stdin = io::stdin();
    let input = &mut String::new();
    let _ = stdin.read_line(input);
    let c: char = input.to_lowercase().chars().nth(0).unwrap();
    return c;
}

fn choose_word() -> String {

    let data = fs::read_to_string("words.txt").expect("Unable to read file");
    let word_list: Vec<&str> = data.split(',').collect();
    let index = rand::thread_rng().gen_range(0, word_list.len());

    return String::from(word_list[index]);
}