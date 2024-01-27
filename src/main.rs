use std::{
    env,
    io::{stdin, Read},
    process::exit,
};

use mrkc::markov_chain;

const DEFAULT_NUMBER_OF_WORDS: i32 = 1000;

fn parse_number_of_words() -> i32 {
    let args: Vec<String> = env::args().collect();
    let number_of_words: i32 = args[1].parse().unwrap_or(DEFAULT_NUMBER_OF_WORDS);

    number_of_words
}

fn main() {
    let mut text = String::new();
    stdin().read_to_string(&mut text).unwrap();

    let number_of_words = parse_number_of_words();

    println!("{}", number_of_words);

    if text.is_empty() {
        println!("Please provide a valid text");
        exit(1);
    }

    println!("{}", markov_chain(text, number_of_words));
}
