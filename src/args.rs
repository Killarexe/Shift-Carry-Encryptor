use std::{path::PathBuf, str::FromStr};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input string/file.
    input: String,
    /// Direction.
    direction: String,
    /// Number of iterations.
    pub iterations: u8,
    /// Output file.
    pub output_path: Option<PathBuf>
}

impl Args {
    pub fn get_input(&mut self) -> Vec<u8> {
        match std::fs::read(self.input.clone())  {
            Ok(bytes) => {
                self.output_path.get_or_insert(PathBuf::from_str(self.input.as_str()).unwrap());
                bytes
            },
            Err(_) => {
                self.input.clone().into_bytes()
            }
        }
    }

    pub fn get_direction(&self) -> Option<bool> {
        if self.direction.eq_ignore_ascii_case("left") {
            Some(false)
        } else if self.direction.eq_ignore_ascii_case("right") {
            Some(true)
        } else {
            None
        }
    }
}
