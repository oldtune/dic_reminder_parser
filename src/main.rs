use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines, Read},
    path::{Iter, Path},
};

use rusqlite::Connection;

fn main() -> Result<(), io::Error> {
    let connection = Connection::open("./dictionary.db").unwrap();
    let mut final_result: Vec<LineFormat> = Vec::new();
    let mut current = LineFormat::new();
    let mut rdr = csv::Reader::from_path("./input.csv")?;
    for result in rdr.records() {
        if let Ok(record) = result {
            let first_field = record.get(0);
            if let Some(value) = first_field {
                if let Ok(some_num) = value.parse::<usize>() {
                    current.number = some_num;
                } else if (current.word.is_empty()) {
                    current.word = value.to_string();
                } else if (current.word_type.is_empty()) {
                    current.word_type = value.to_string();
                } else {
                    current.meaning.push_str(value);
                }
            }
            let second_field = record.get(1);
            if let Some(value) = second_field {
                if (current.word.is_empty()) {
                    current.word = value.to_string();
                }
            }
            let third_field = record.get(2);
            if let Some(value) = third_field {
                if (current.word_type.is_empty()) {
                    current.word_type = value.to_string();
                }
            }
            let fourth_field = record.get(3);
            if let Some(value) = fourth_field {
                if (current.pronounce.is_empty()) {
                    current.pronounce = value.to_string();
                }
            }
            let fifth_field = record.get(4);
            if let Some(value) = fifth_field {
                if current.meaning.is_empty() {
                    current.meaning = value.to_string();
                }
            }
            let sixth_field = record.get(5);
            if let Some(value) = sixth_field {
                current.meaning.push_str(value);
            }

            if current.is_complete() {
                connection
                    .execute(
                        "INSERT INTO WORDS VALUES(?1, ?2, ?3, ?4)",
                        (
                            &current.word,
                            &current.word_type,
                            &current.pronounce,
                            &current.meaning,
                        ),
                    )
                    .unwrap();
                current = LineFormat::new();
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct LineFormat {
    number: usize,
    word: String,
    word_type: String,
    pronounce: String,
    meaning: String,
}

impl LineFormat {
    pub fn new() -> Self {
        LineFormat {
            number: 0,
            word: "".to_string(),
            word_type: "".to_string(),
            pronounce: "".to_string(),
            meaning: "".to_string(),
        }
    }

    pub fn is_complete(&self) -> bool {
        self.number > 0
            && !self.word.is_empty()
            && !self.word_type.is_empty()
            && !self.pronounce.is_empty()
            && !self.meaning.is_empty()
    }
}
