use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines, Read},
    path::{Iter, Path},
};

fn main() -> Result<(), io::Error> {
    let lines = read_line("./input.csv")?;
    for line in lines {
        if let Ok(line) = line {}
    }
    return Ok(());
}

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

pub fn read_line<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    let buffer = BufReader::new(file);
    Ok(buffer.lines())
}
