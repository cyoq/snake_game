use std::fs::File;
use std::io;
use std::path::Path;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

fn write_best_score(filename: &str, best: u32) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_u32::<BigEndian>(best).unwrap();
    Ok(())
}

fn read_best_score(filename: &str) -> io::Result<u32> {
    if !file_exists(filename) {
        write_best_score(filename, 0).ok();
    }

    let mut file = File::open(filename)?;
    let best_score = file.read_u32::<BigEndian>()?;
    Ok(best_score)
}

fn file_exists(filename: &str) -> bool {
    Path::new(filename).exists()
}

pub struct Score<'a> {
    pub current: u32,
    pub best: u32,
    filename: &'a str,
}

impl<'a> Score<'a> {
    pub fn new() -> Self {
        let filename = "score";
        let best_score = read_best_score(filename).expect("Error occurred with files!");
        Self {
            current: 0,
            best: best_score,
            filename,
        }
    }

    pub fn increment(&mut self) {
        self.current += 1;
    }

    pub fn zero(&mut self) {
        self.current = 0;
    }

    pub fn update(&mut self) {
        if self.current > self.best {
            println!("New best score has been set!");
            self.best = self.current;
            write_best_score(self.filename, self.best).ok();
        }
    }
}