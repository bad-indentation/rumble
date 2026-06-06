use std::{collections::HashSet, fs, process::exit};

use clap::Parser;

/// Loads words from the given filepath
/// Returns Err variant if file cannot be read
fn load_words(path: &str) -> Result<HashSet<String>, std::io::Error> {
    let mut words = HashSet::new();
    let lines = fs::read_to_string(path)?;

    for word in lines.split_whitespace() {
        words.insert(String::from(word).to_lowercase());
    }

    Ok(words)
}

/// Returns the set of all unique prefixes within the set.
fn get_prefixes(words: &HashSet<String>) -> HashSet<String> {
    let mut prefixes = HashSet::new();

    for word in words {
        for end in 1..=word.len() {
            prefixes.insert(String::from(&word[0..end]));
        }
    }
    

    prefixes
}

/// Represents a potential word as it is being constructed via DFS
struct PathBuilder {
    word: String,
    letters_available: Vec<char>,
}

impl PathBuilder {
    /// Creates a new path from the given scrambled word
    fn from(scrambled: &str) -> Self {
        PathBuilder { word: String::new(), letters_available: scrambled.chars().collect() }
    }

    fn new(word: String, letters_available: Vec<char>) -> Self {
        PathBuilder { word, letters_available }
    }
}

/// Returns all valid prefixes that can be made using the scrambled letters in `word_path`
/// TODO: get rid of all the clones
fn get_words(word_path: PathBuilder, valid_prefixes: &HashSet<String>) -> HashSet<String> {
    if word_path.letters_available.is_empty() {
        return HashSet::from([word_path.word]);
    }

    let mut found: HashSet<String> = HashSet::new();
    let mut new_path;

    for (i, letter) in word_path.letters_available.iter().enumerate() {
        let mut new_word = word_path.word.clone();
        new_word.push(*letter);

        if valid_prefixes.contains(&new_word) {
            let mut new_letters = word_path.letters_available.clone();
            new_letters.remove(i);
            
            found.insert(new_word.clone());

            new_path = PathBuilder::new(
                new_word.to_string(),
                new_letters,
            );

            found.extend(get_words(new_path, valid_prefixes).iter().cloned());
        }
    }

    found
}


/// Configuration for command line args Parser
/// Based on example in Clap documentation:
/// https://docs.rs/clap/latest/clap/#example
#[derive(Parser)]
#[command(version, about = "A blazingly fast Jumble solver written in Rust", long_about = None)]
pub struct Config {
    /// The scrambled word you want to unscramble
    scrambled: String,

    /// Whether to include words that don't use all available letters
    #[arg(short = 'p', long, default_value_t = false)]
    include_partial: bool,

    /// Print debugging details to stderr
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

impl Config {
    pub fn new(scrambled: String, verbose: bool, include_partial: bool) -> Self {
        Self { scrambled, verbose, include_partial }
    } 
}

fn eprintln_if_verbose(msg: &str, verbose: bool) {
    if verbose {
        eprintln!("{}", msg);
    }
}

pub fn run(config: Config) {
    eprintln_if_verbose("Loading wordlist...", config.verbose);
    let valid_words = match load_words("public/wordlist.txt") {
        Ok(words) => words,
        Err(e) => {
            eprintln!("Problem loading words: {:?}", e);
            eprintln!("Hint: this error is likely because you're in the wrong directory!");
            exit(1);
        }
    };

    eprintln_if_verbose("Precomputing prefixes...", config.verbose);
    let valid_prefixes = get_prefixes(&valid_words);

    let mut words = Vec::new();

    eprintln_if_verbose("Searching...", config.verbose);
    for word in get_words(PathBuilder::from(&config.scrambled), &valid_prefixes) {
        if valid_words.contains(&word) && (word.len() == config.scrambled.len() || config.include_partial) {
            words.push(word);
        }
    }
    
    if !words.is_empty() {
        words.sort();
        words.iter().for_each(|word| println!("{}", word));
        eprintln_if_verbose(&format!("Found {} words.", words.len()), config.verbose);
        exit(0);
    }

    eprintln!("Sorry, couldn't find any words. :(")

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_loading() {
        let mut expected: HashSet<String> = HashSet::new();
        expected.extend(["word1", "word2", "anotherword"].map(String::from));
        assert_eq!(load_words("public/test_list.txt").unwrap(), expected);
    }

    #[test]
    fn test_failed_word_loading() {
        assert!(match load_words("public/nonexistent.txt") {
            Ok(_) => false,
            Err(_) => true,
        });
    }

    #[test]
    fn test_get_prefixes() {
        let mut words = HashSet::new();
        words.extend(["apple", "banana"].map(String::from));

        let mut expected_prefixes = HashSet::new();
        expected_prefixes.extend(
            [
                "a", "ap", "app", "appl", "apple", "b", "ba", "ban", "bana", "banan", "banana",
            ]
            .map(String::from),
        );

        assert_eq!(get_prefixes(&words), expected_prefixes);
    }

    #[test]
    fn test_get_words() {
        let valid_prefixes = get_prefixes(&load_words("public/test_list.txt").unwrap());
        let results = get_words(PathBuilder::from("2owrd"), &valid_prefixes);
    

        assert_eq!(results, HashSet::from(["w", "wo", "wor", "word", "word2"].map(String::from)));
    } 
}
