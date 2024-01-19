use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut result :HashSet<&'a str> = HashSet::new();
    println!("{}", word);

    for possible_anagram in possible_anagrams {
        let mut is_word_a_match = true;
        if possible_anagram.chars().count() != word.chars().count() {
                continue;
        }
        
        for c in word.chars() {
            if !possible_anagram.contains(c) {
                is_word_a_match = false
            }
        }

        if is_word_a_match
        {
            result.insert(possible_anagram);
        }
        
        println!("result in loop {:?}", result);
    }

    println!("{:?}", result);
    result
}
