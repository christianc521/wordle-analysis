use std::{collections::HashMap, iter};

use crate::vowels;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PairingPattern {
    VowelToConsonant,
    VowelToVowel,
    ConsonantToConsonant,
    ConsonantToVowel,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LetterPair {
    letter: char,
    follower: char,
}

impl LetterPair {
    fn new(letter: char, follower: char) -> Self {
        LetterPair { letter, follower }
    }
}

pub struct IndexedLetterStats {
    pub alphabet: HashMap<LetterPair, i32>,
    pub patterns: HashMap<PairingPattern, i32>,
    pub vowels: i32,
    pub consonants: i32,
    pub vowel_percentage: f32,
}

impl IndexedLetterStats {
    pub fn new() -> Self {
        let alphabet: HashMap<LetterPair, i32> = HashMap::new();
        let patterns: HashMap<PairingPattern, i32> = HashMap::new();

        IndexedLetterStats {
            alphabet,
            patterns,
            vowels: 0,
            consonants: 0,
            vowel_percentage: 0.0,
        }
    }

    pub fn add_pair(&mut self, pair: LetterPair) {
        *self.alphabet.entry(pair).or_insert(1) += 1;
    }

    pub fn add_pattern(&mut self, pair: PairingPattern) {
        *self.patterns.entry(pair).or_insert(1) += 1;
    }
}

pub fn count_vowels(dictionary: &Vec<String>) -> IndexedLetterStats {
    let mut stats = IndexedLetterStats::new();
    let mut previous_char: char = '.';
    let mut word_indexes: Vec<HashMap<char, i32>> = vec![HashMap::new(); 5];

    // iterate through each letter in each word, increment vowels accordingly
    for word in dictionary {
        let chars: Vec<char> = word.chars().collect();
        let word_len = chars.len();

        for index in 0..word_len {
            let c = chars[index];
            *word_indexes[index].entry(c).or_insert(1) += 1;

            if index < word_len - 1 {
                match c {
                    'a' | 'e' | 'i' | 'o' | 'u' => {
                        stats.vowels += 1;

                        if index > 0 && previous_char != '.' {
                            if "aeiou".contains(previous_char) {
                                stats.add_pattern(PairingPattern::VowelToVowel);
                            } else {
                                stats.add_pattern(PairingPattern::ConsonantToVowel);
                            }

                            let pair = LetterPair::new(previous_char, c);
                            stats.add_pair(pair);
                        }
                    }
                    _ if c.is_alphabetic() => {
                        stats.consonants += 1;

                        if index > 0 && previous_char != '.' {
                            if "aeiou".contains(previous_char) {
                                stats.add_pattern(PairingPattern::VowelToConsonant);
                            } else {
                                stats.add_pattern(PairingPattern::ConsonantToConsonant);
                            }

                            let pair = LetterPair::new(previous_char, c);
                            stats.add_pair(pair);
                        }
                    }
                    _ => (),
                }
            }
            previous_char = c;
        }
    }

    println!("First letter rankings:");
    let mut rank = 1;
    let mut ranked_letters: Vec<(&char, &i32)> = word_indexes[4].iter().clone().collect();
    ranked_letters.sort_by(|a, b| b.1.cmp(&a.1));
    for (letter, count) in ranked_letters {
        println!("{rank}. {letter} ({count})");
        rank += 1;
    }

    stats.vowel_percentage = stats.vowels as f32 / (stats.vowels + stats.consonants) as f32;

    stats
}
