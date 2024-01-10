mod args;

#[cfg(test)]
mod tests;

use std::{fs::OpenOptions, io::Write, path::PathBuf};
use args::*;
use clap::Parser;

pub fn shift_carry_left(value: u8, iterations: u8) -> u8 {
    let mut result: u8 = value;
    for _ in 0..(iterations){
        let has_carry: bool = result >= 128;
        result = result << 1;
        if has_carry {
            result += 1;
        }
    }
    result
}

pub fn shift_carry_right(value: u8, iterations: u8) -> u8 {
    let mut result: u8 = value;
    for _ in 0..(iterations){
        let has_carry: bool = (result & 1) == 1;
        result = result >> 1;
        if has_carry {
            result += 128;
        }
    }
    result
}

pub fn process_input(input: Vec<u8>, iterations: u8, direction: bool, invert: bool) -> Vec<u8>{
    if direction {
        return input.iter().map(|&value| if invert {
            !shift_carry_left(value, iterations)
        } else {
            shift_carry_left(value, iterations)
        }).collect();
    }
    input.iter().map(|&value| if invert { 
        shift_carry_right(!value, iterations)
    } else {
        shift_carry_right(value, iterations)
    }).collect()
}

pub fn process_output(output: Vec<u8>, output_path: Option<PathBuf>) {
    if let Some(path) = output_path {
        match OpenOptions::new().write(true).create(true).open(path) {
            Ok(mut file) => {
                file.write_all(&output).expect("Failed to write into file...");
            }
            Err(err) => {
                println!("Failed to create/write to file:\n{:#?}", err);
            }
        }
    } else {
        output.iter().for_each(|&value| print!("{}", value as char));
    }
}

fn main() {
    let mut args: Args = Args::parse();
    match args.get_direction() {
        Some(direction) => {
            let output: Vec<u8> = process_input(args.get_input(), args.iterations, direction, args.get_invert_output());
            process_output(output, args.output_path);
        },
        None => {
            println!("No direction found! Please add direction to process! --help for help...");
        }
    } 
    println!("Program terminated!");
}
