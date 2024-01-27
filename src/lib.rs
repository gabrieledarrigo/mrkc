use rand::{self, seq::SliceRandom};
use std::collections::HashMap;

const EMPTY: &str = "";

pub fn markov_chain(text: String, number_words: i32) -> Vec<String> {
    let mut word_1 = EMPTY;
    let mut word_2 = EMPTY;

    let mut possibles: HashMap<(&str, &str), Vec<&str>> = HashMap::new();

    // Fill the map of possible words
    for line in text.lines() {
        for word in line.split_whitespace() {
            let entry = possibles.entry((word_1, word_2)).or_insert(vec![]);
            entry.push(word.trim());

            word_1 = word_2;
            word_2 = word.trim();
        }
    }

    // Avoid empty possibles lists at end of input
    possibles
        .entry((word_1, word_2))
        .and_modify(|v| v.push(EMPTY));

    possibles
        .entry((word_1, EMPTY))
        .and_modify(|v| v.push(EMPTY));

    let mut with_capital: Vec<(&str, &str)> = vec![];

    // Compute the list of possible starting words
    for key in possibles.keys() {
        let (first, _) = *key;

        if first
            .chars()
            .next()
            .map(|c| c.is_ascii_uppercase())
            .unwrap_or(false)
        {
            with_capital.push(*key);
        }
    }

    (word_1, word_2) = *with_capital.choose(&mut rand::thread_rng()).unwrap();

    let mut output = vec![word_1, word_2];

    // Fill the output vector
    for _ in 0..number_words {
        match possibles.get(&(word_1, word_2)) {
            Some(words) => {
                let word = words.choose(&mut rand::thread_rng()).unwrap();

                output.push(*word);
                word_1 = word_2;
                word_2 = word;
            }
            None => {}
        }
    }

    output.iter().map(|&word| word.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markov_chain() {
        let text = String::from("Lorem Ipsum è un testo segnaposto utilizzato nel settore della tipografia e della stampa.");
        let number_words = 10;

        let result = markov_chain(text.clone(), number_words);

        assert_eq!(result.len(), number_words as usize + 2);

        for word in &result {
            assert!(
                text.contains(word),
                "The output contains a word not present in the input text: {}",
                word
            );
        }

        for window in result.windows(2) {
            let phrase = format!("{} {}", window[0], window[1]);

            assert!(
                text.contains(&phrase),
                "Output contains a phrase not present in the input text: {}",
                phrase
            );
        }
    }
}
