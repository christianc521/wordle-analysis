use std::collections::HashMap;

pub struct BigramUtility {
    start_position: u8,
    bigrams: HashMap<String, u16>,
}

impl BigramUtility {
    pub fn collect_bigrams(start_pos: usize, word_list: &Vec<String>) -> Vec<String> {
        let mut bigram_list: Vec<String> = Vec::new();

        for word in word_list {
            let trimmed_word = word.trim();
            if trimmed_word.len() >= start_pos + 2 {
                let bigram = &trimmed_word[start_pos..start_pos + 2];
                let bigram_str = bigram.to_string();
                bigram_list.push(bigram_str);
            }
        }

        bigram_list
    }

    pub fn count_bigrams(start_pos: i32, word_list: &Vec<String>) -> HashMap<String, i32> {
        let mut bigrams: HashMap<String, i32> = HashMap::new();
        let start_index = start_pos as usize;

        for word in word_list {
            let trimmed_word = word.trim();
            if trimmed_word.len() >= start_index + 2 {
                let bigram = &trimmed_word[start_index..start_index + 2];
                let bigram_str = bigram.to_string();
                *bigrams.entry(bigram_str).or_insert(0) += 1;
            }
        }

        bigrams
    }
}
