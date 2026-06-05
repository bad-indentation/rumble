use std::{collections::HashSet, fs};

/// Loads words from the given filepath
/// Returns Err variant if file cannot be read
pub fn load_words(path: &str) -> Result<HashSet<String>, std::io::Error> {
    let mut words = HashSet::new();
    let lines = fs::read_to_string(path)?;
    
    for word in lines.split_whitespace() {
        words.insert(String::from(word).to_lowercase());
    }

    return Ok(words); 
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
}
