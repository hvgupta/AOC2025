use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 The idea over here was to create a python-like generator that can be used to read words from a file.
*/

pub struct FileLineIterator {
    pub reader: Vec<String>,
}
impl FileLineIterator {
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file)
            .lines()
            .collect::<Result<Vec<String>, _>>().unwrap();

        Ok(FileLineIterator { reader })
    }

    pub fn lines(&self) -> &[String] {
        &self.reader
    }
}
