use std::collections::HashSet;

/// Load words from the given filepath
pub fn load_words(path: &str) -> HashSet<String> {
   return HashSet::new(); 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_loading() {
        let mut expected: HashSet<String> = HashSet::new();
        expected.extend(["word1", "word2", "anotherword"].map(String::from));
        assert_eq!(load_words("public/test_list.txt"), expected);
    }
}
