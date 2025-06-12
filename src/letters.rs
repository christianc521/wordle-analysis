use std::collections::HashMap;

pub struct PositionData {
    pub letters: HashMap<char, i32>,
    pub index: usize,
}

impl PositionData {
    pub fn analyze_position(dictionary: &Vec<String>, position: usize) -> PositionData {
        let letters: HashMap<char, i32> = HashMap::new();

        let mut data = PositionData {
            letters,
            index: position,
        };

        for word in dictionary {
            let letter = word
                .chars()
                .nth(position)
                .expect("Failed to reach position in word.");

            *data.letters.entry(letter).or_insert(1) += 1;
        }

        data
    }

    pub fn analyze_bigram(
        bigrams: &HashMap<String, i32>,
        first_letter: Option<char>,
    ) -> PositionData {
        let mut data = PositionData {
            letters: HashMap::new(),
            index: 1,
        };

        for (bigram, count) in bigrams {
            let lead_letter = bigram
                .chars()
                .nth(0)
                .expect("Failed to reach bigram 2nd letter.");
            match first_letter {
                Some(c) => {
                    if c == lead_letter {
                        let second_letter = bigram
                            .chars()
                            .nth(1)
                            .expect("Failed to reach bigram 2nd letter.");

                        *data.letters.entry(second_letter).or_insert(count.clone()) += count;
                    }
                }
                None => {}
            }
        }

        data
    }
}
