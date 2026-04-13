use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use crate::source::LogSource;

pub struct FileSource {
    lines: Lines<BufReader<File>>,
}

impl FileSource {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).expect("Failed to open file");

        let reader = BufReader::new(file);

        Self {
            lines: reader.lines(),
        }
    }
}

impl LogSource for FileSource {
    fn read_line(&mut self) -> Option<String> {
        self.lines.next()?.ok()
    }
}
