use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();

    for possible_anagram in possible_anagrams {
        let mut is_word_a_match = true;
        if possible_anagram.chars().count() != word.chars().count() {
            continue;
        }

        if possible_anagram.to_lowercase() == word.to_lowercase() {
            continue;
        }

        let mut processed = HashSet::new();
        for c in word.to_lowercase().chars() {
            
            processed.contains(&c);
            if !possible_anagram.to_lowercase().contains(c) {
                is_word_a_match = false
            }

            processed.insert(c);
        }

        if is_word_a_match {
            result.insert(possible_anagram);
        }
    }

    result
}
