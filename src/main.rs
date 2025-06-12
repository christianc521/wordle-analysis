mod bigram_util;
mod file_writer;
mod letters;
mod visuals;
mod vowels;
use core::f64;
use std::{collections::HashMap, fs};

use letters::PositionData;
use plotters::style::{
    full_palette::{ORANGE, PURPLE},
    BLUE, GREEN, RED,
};
use vowels::PairingPattern;

struct WordleDictionary {
    all_words: Vec<String>,
}

impl WordleDictionary {
    fn new(file_path: &str) -> Self {
        let content = fs::read_to_string(file_path).expect("Failed to read text file.");
        let words = content.lines().map(|line| line.to_string()).collect();

        WordleDictionary { all_words: words }
    }
}

fn main() {
    let dictionary = WordleDictionary::new("words.txt");

    // Loop over all the words and letters to create overall vowel-consonant pie chart
    let indexed_vowels = vowels::count_pair_pattern(&dictionary.all_words);
    let pairing_pattern = indexed_vowels.patterns;
    let mut pairing_data: Vec<f64> = vec![0.0; 4];
    let mut total_count = 0;
    for (pattern, count) in pairing_pattern {
        match pattern {
            PairingPattern::VowelToVowel => pairing_data[0] = count as f64,
            PairingPattern::VowelToConsonant => pairing_data[1] = count as f64,
            PairingPattern::ConsonantToConsonant => pairing_data[2] = count as f64,
            PairingPattern::ConsonantToVowel => pairing_data[3] = count as f64,
        }
        total_count += count;
    }

    let pairing_percent: Vec<f64> = pairing_data
        .into_iter()
        .map(|d| d / total_count as f64)
        .collect();

    let title = "Linear Letter to Vowel/Consonant Change".to_owned();
    let pie_filename = "letter-change-pie".to_owned();
    let labels: Vec<String> = vec![
        "Vowel to Vowel".to_string(),
        "Vowel to Consonant".to_string(),
        "Consonant to Consonant".to_string(),
        "Consonant to Vowel".to_string(),
    ];

    let _ = visuals::draw_vowel_pie(
        &pairing_percent,
        &[BLUE, ORANGE, GREEN, PURPLE],
        title,
        pie_filename,
        &labels,
    );

    let mut all_bigrams: HashMap<String, i32> = HashMap::new();
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    // Create a visual of a bigram at every index
    for n in 0..4 {
        // Create a bigram bar chart
        let bigram_freq = bigram_util::BigramUtility::count_bigrams(n, &dictionary.all_words);
        all_bigrams.extend(bigram_freq.clone().into_iter());

        let bigram_list: Vec<String> = bigram_freq.keys().cloned().collect();
        let title = "Bigram Frequency at Postition ".to_owned() + &n.to_string();
        let filename = "bigram-freq-".to_owned() + &n.to_string();

        let _ = visuals::draw_bigram_bar(bigram_freq, &title, filename, Some(20));

        // Create a vowel-consonant pie chart at index n
        let vowel_stats = vowels::count_vowels(&bigram_list);

        let vowel_percent = vowel_stats.vowel_percentage as f64 * 100.0;
        let consonant_percent: f64 = 100.0 - vowel_percent as f64;
        let data: Vec<f64> = vec![vowel_percent, consonant_percent];

        let title = "Vowel and Consonant Ratio at Position ".to_owned() + &n.to_string();
        let pie_filename = "vowel-percent-".to_owned() + &n.to_string();
        let labels: Vec<String> = vec!["Vowels".to_string(), "Consonants".to_string()];
        let _ = visuals::draw_vowel_pie(&data, &[BLUE, ORANGE], title, pie_filename, &labels);

        // Retrieve single letter data at each position
        let letter_data = PositionData::analyze_position(&dictionary.all_words, n as usize);
        let file_name = format!("data-reports/letter-freqency-pos-{}", &n);
        file_writer::write_letter_data(letter_data, n as usize, file_name, None);

        if n == 3 {
            let letter_data = PositionData::analyze_position(&dictionary.all_words, 1 + n as usize);
            let file_name = format!("data-reports/letter-freqency-pos-{}", 1 + &n);
            file_writer::write_letter_data(letter_data, n as usize, file_name, None);
        }

        // Analyze all the bigrams
        for char in alphabet.chars() {
            let file_name = format!("data-reports/letter-followers/{}-followers", char);
            let data = letters::PositionData::analyze_bigram(&all_bigrams, Some(char));

            let title = format!("Occurences of Letter following '{}'\n", char);
            file_writer::write_letter_data(
                data,
                n as usize,
                file_name.clone(),
                Some(title.clone()),
            );
        }
    }

    let mut alphabet_hash: HashMap<String, i32> = HashMap::new();
    // Run analysis after full bigram collection
    for char in alphabet.chars() {
        let mut string_hash: HashMap<String, i32> = HashMap::new();
        let barchart_data = letters::PositionData::analyze_bigram(&all_bigrams, Some(char));

        for (letter, count) in barchart_data.letters {
            *string_hash.entry(letter.to_string()).or_insert(count) += count;
            *alphabet_hash.entry(letter.to_string()).or_insert(count) += count;
        }

        let title = format!("Occurences of Letter following '{}'", char);
        let file_name = format!("letter-followers/{}-followers", char);
        let _ = visuals::draw_bigram_bar(string_hash, &title, file_name.clone(), None);
        println!("{} added.", &file_name);
    }

    let title = format!("Occurence's of Second Letter in All Bigrams");
    let file_name = format!("second-letter-bar");
    let _ = visuals::draw_bigram_bar(alphabet_hash, &title, file_name, None);
}
