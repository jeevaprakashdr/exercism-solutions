use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let word_lowercase = word.to_lowercase();
    let mut word_lowercase_vec = word_lowercase.chars().collect::<Vec<char>>();
    word_lowercase_vec.sort();

    possible_anagrams
        .iter()
        .filter(|x| x.chars().count() == word.chars().count())
        .filter(|x| x.to_lowercase() != word_lowercase)
        .filter(|anagrams| {
            let mut sorted_anagram = anagrams.to_lowercase().chars().collect::<Vec<char>>();
            sorted_anagram.sort();
            sorted_anagram == word_lowercase_vec
        })
        .map(|x| *x)
        .collect()
}
