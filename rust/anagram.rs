use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let sorted_word = sort_string(&lowercase_word);
    let mut result: HashSet<&str> = HashSet::new();

    for possible_anagram in possible_anagrams {
        let lowercase_possible_anagram = possible_anagram.to_lowercase();

        if lowercase_word == lowercase_possible_anagram {
            continue;
        }

        let sorted_possible_anagram = sort_string(&lowercase_possible_anagram);

        if sorted_word == sorted_possible_anagram {
            result.insert(possible_anagram);
        }
    }

    result
}

fn sort_string(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}
