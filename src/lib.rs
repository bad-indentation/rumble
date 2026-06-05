use std::{collections::HashSet, fs};

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
    fn new(scrambled: &str) -> Self {
        PathBuilder { word: String::new(), letters_available: scrambled.chars().collect() }
    }
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
}
