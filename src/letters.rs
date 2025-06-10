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
}
