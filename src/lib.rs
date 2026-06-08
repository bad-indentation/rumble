use std::collections::{HashSet, HashMap};
use std::fs;
use std::process;

use clap::Parser;

/// Helper function. Returns true if word does not contain any letters
/// not in `letters`
fn no_invalid_letters(word: &str, letters: &HashSet<char>) -> bool {
    for letter in word.chars() {
        if !letters.contains(&letter) {
            return false;
        }
    }
    true
}

/// Returns the count for each letter in the given word.
///
/// Based on example code from the HashMap documentation.
fn get_letter_counts(word: &str) -> HashMap<char, usize> {
    let mut counter = HashMap::new();

    for letter in word.chars() {
        counter.entry(letter).and_modify(|ct| *ct += 1).or_insert(1);
    }

    counter
}

/// Returns true if `word` is an anagram of the string whose letter counts
/// are given by `count`
/// If `partial` is true, `word` need not use every letter in count 
/// to be considered a (partial) anagram
fn is_anagram_of(word: &str, target: &HashMap<char, usize>, partial: bool) -> bool {
    let mut candinate_count = HashMap::new();

    for letter in word.chars() {
        candinate_count.entry(letter).and_modify(|ct| *ct += 1).or_insert(1);
        
        if *target.get(&letter).unwrap_or(&0) < *candinate_count.get(&letter).expect("violates invariant") {
            return false;
        } 
        
    }

    partial || *target == candinate_count
}

/// Loads words from the given filepath
///
/// Only uses words with lengths <= `max_len` and contain only `letters`
///
/// Returns Err variant if file cannot be read
fn load_words(
    path: &str,
    max_len: usize,
    letters: &HashSet<char>,
) -> Result<HashSet<String>, std::io::Error> {
    let mut words = HashSet::new();
    let lines = fs::read_to_string(path)?;

    for word in lines.split_whitespace() {
        if word.len() <= max_len && no_invalid_letters(word, letters) {
            words.insert(String::from(word).to_lowercase());
        }
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
        PathBuilder {
            word: String::new(),
            letters_available: scrambled.chars().collect(),
        }
    }

    fn new(word: String, letters_available: Vec<char>) -> Self {
        PathBuilder {
            word,
            letters_available,
        }
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

            new_path = PathBuilder::new(new_word.to_string(), new_letters);

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
        Self {
            scrambled,
            verbose,
            include_partial,
        }
    }
}

fn eprintln_if_verbose(msg: &str, verbose: bool) {
    if verbose {
        eprintln!("{}", msg);
    }
}

pub fn run(config: Config) {
    eprintln_if_verbose("Loading wordlist...", config.verbose);

    let max_len = config.scrambled.len();
    let letters = HashSet::from_iter(config.scrambled.chars());
    let result = load_words("public/wordlist.txt", max_len, &letters);

    let valid_words = match result {
        Ok(words) => words,
        Err(e) => {
            eprintln!("Problem loading words: {:?}", e.to_string());
            eprintln!("Hint: this error is likely because you're in the wrong directory!");
            process::exit(1);
        }
    };

    let target_count = get_letter_counts(&config.scrambled);
    let mut solutions = 0;

    eprintln_if_verbose("Searching...", config.verbose);
    for word in valid_words.iter().filter(|word| is_anagram_of(word, &target_count, config.include_partial)) {
        solutions += 1;
        println!("{word}");
    }

    if solutions == 0 {
        eprintln!("Sorry, couldn't find any solutions. :(");
    } else {
        eprintln!("Found {solutions} solutions.");
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_loading() {
        let mut expected: HashSet<String> = HashSet::new();
        expected.extend(["word1", "word2", "anotherword"].map(String::from));
        assert_eq!(
            load_words(
                "public/test_list.txt",
                100,
                &HashSet::from_iter("abcdefghijklmnopqrstuvwxyz12".chars())
            )
            .unwrap(),
            expected
        );
    }

    #[test]
    fn test_failed_word_loading() {
        assert!(match load_words(
            "public/nonexistent.txt",
            100,
            &HashSet::from(['a', 'b', 'c'])
        ) {
            Ok(_) => false,
            Err(_) => true,
        });
    }

    #[test]
    fn test_get_letter_counts() {
        let word = "banana";
        let expected = HashMap::from([('b', 1), ('a', 3), ('n', 2)]);
        assert_eq!(get_letter_counts(word), expected);
    }

    #[test]
    fn test_true_anagram() {
        let count = get_letter_counts("nagaram");
        let candidate = "anagram";

        assert!(is_anagram_of(candidate, &count, false));
    }
    
    #[test]
    fn test_partial_anagram_not_full_anagram() {
        let count = get_letter_counts("anagram");
        let candidate = "gram";

        assert!(!is_anagram_of(candidate, &count, false));
    }

    #[test]
    fn test_partial_anagram() {
        let count = get_letter_counts("anagram");
        let candidate = "gram";

        assert!(is_anagram_of(candidate, &count, true));
    }

    #[test]
    fn test_no_anagram() {
        let count = get_letter_counts("anagram");
        let candidate = "banana";

        assert!(!is_anagram_of(candidate, &count, true));
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
        let max_len = 100;
        let letters = HashSet::from_iter("abcdefghijklmnopqrstuvwxyz12".chars());
        let valid_prefixes =
            get_prefixes(&load_words("public/test_list.txt", max_len, &letters).unwrap());
        let results = get_words(PathBuilder::from("2owrd"), &valid_prefixes);

        assert_eq!(
            results,
            HashSet::from(["w", "wo", "wor", "word", "word2"].map(String::from))
        );
    }
}
