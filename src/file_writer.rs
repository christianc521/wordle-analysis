use super::letters::PositionData;
use std::fs::File;
use std::io::{LineWriter, Write};

pub fn write_letter_data(
    data: PositionData,
    index: usize,
    filename: String,
    title: Option<String>,
) {
    let file = File::create(filename).expect("Failed to create file.");
    let mut file = LineWriter::new(file);

    let mut header = format!("Letter Frequency at Position {}\n", index);
    if title.is_some() {
        header = title.expect("Failed to unwrap title.");
    }

    let _ = file.write_all(header.as_bytes());

    // Sort the letters by value (occurrences)
    let mut ranked_letters: Vec<(&char, &i32)> = data.letters.iter().clone().collect();
    ranked_letters.sort_by(|a, b| b.1.cmp(&a.1));

    // Write each key-value pair as a new line
    let mut rank = 1;
    for (letter, count) in ranked_letters {
        let line = format!("{}. {} ({})\n", &rank, letter, count);
        let _ = file.write_all(line.as_bytes());
        rank += 1;
    }

    file.flush().expect("Failed to close file.");
}
