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

        for c in word.to_lowercase().chars() {
            
            let char_count_in_word = word.to_lowercase().chars().filter(|x| x.eq(&c)).count();
            let char_count_in_anagram = possible_anagram.to_lowercase().chars().filter(|x| x.eq(&c)).count();

            if !possible_anagram.to_lowercase().contains(c) || char_count_in_anagram > char_count_in_word{
                is_word_a_match = false
            }
        }

        if is_word_a_match {
            result.insert(possible_anagram);
        }
    }

    result
}
