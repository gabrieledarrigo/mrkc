use std::{
    env,
    io::{stdin, Error, Read},
    process::exit,
};

use mrkc::markov_chain;

const DEFAULT_NUMBER_OF_WORDS: i32 = 1000;

/// Parses the command line argument to determine the number of words to generate.
/// If no argument is provided, the default number of words is used.
///
/// # Arguments
///
/// None
///
/// # Returns
///
/// * `i32` - The number of words to generate.
fn parse_number_of_words() -> i32 {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() || args.len() == 1 {
        return DEFAULT_NUMBER_OF_WORDS;
    }

    let number_of_words: i32 = args[1].parse().unwrap_or(DEFAULT_NUMBER_OF_WORDS);

    number_of_words
}

/// Reads the input from the user
fn from_stdin() -> Result<String, Error> {
    let mut text = String::new();
    stdin().read_to_string(&mut text)?;

    Ok(text)
}

/// Reads input from the user, parses the command line argument for the number of words,
/// and generates a Markov chain based on the input text.
///
/// # Arguments
///
/// None
///
/// # Returns
///
/// None
fn main() -> Result<(), Error> {
    let text = from_stdin()?;
    let number_of_words = parse_number_of_words();

    if text.is_empty() {
        println!("Please provide a valid text");
        exit(1);
    }

    println!("{}", markov_chain(text, number_of_words));
    Ok(())
}
