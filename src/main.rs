use std::{
    fs::File,
    io::{self, BufRead, Read},
    path::{Iter, Path},
};

fn main() {
    let mut file = File::open("./input.txt").unwrap();
    let mut buffer = vec![];
    let content = file.read_to_end(&mut buffer).unwrap();
    let line_count = count_line(&buffer);
    println!("{}", line_count);
    // println!("{}", String::from_utf8_lossy(&buffer));
}

fn count_line(text: &[u8]) -> usize {
    let mut counter = 0;
    for i in 0..text.len() {
        if text[i] == 0xa {
            counter += 1;
        }
    }
    counter
}

pub struct LineFormat<'a> {
    number: usize,
    word: &'a str,
    word_type: &'a str,
    pronounce: &'a str,
    meaning: &'a str,
}
impl<'a> LineFormat<'a> {
    pub fn new() -> Self {
        LineFormat {
            number: 0,
            word: "",
            word_type: "",
            pronounce: "",
            meaning: "",
        }
    }
}

pub fn read_line<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    todo!()
}
