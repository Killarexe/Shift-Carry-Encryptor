use std::{fs::{self, OpenOptions}, io::Write};

fn shift_left(value: u8, iteration: u8) -> u8 {
    let mut result: u8 = value;
    for _ in 0..(iteration){
        let has_carry: bool = result >= 128;
        result = result << 1;
        if has_carry {
            result += 1;
        }
    }
    result
}

fn shift_right(value: u8, iteration: u8) -> u8 {
    let mut result: u8 = value;
    for _ in 0..(iteration){
        let has_carry: bool = (result & 1) == 1;
        result = result >> 1;
        if has_carry {
            result += 128;
        }
    }
    result
}

fn encrypt(input: Vec<u8>, iterations: u8) -> Vec<u8> {
    let mut result: Vec<u8> = vec![0; input.len()];
    for i in 0..input.len() {
        result[i] = !shift_left(input[i], iterations);
    }
    result
}

fn decrypt(input: Vec<u8>, iterations: u8) -> Vec<u8> {
    let mut result: Vec<u8> = vec![0; input.len()];
    for i in 0..input.len() {
        result[i] = shift_right(!input[i], iterations);
    }
    result
}

fn main() {
    //program <file|contents> <e|d> <iterations> <output_file>(optional)
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4{
        println!("Usage: file_encryptor <file|contents> <left|right> <iterations> <output_file>(optional)");
        return;
    }
    let need_file_output: bool = args.len() > 4;
    let encrypt_op: bool = args[2].eq_ignore_ascii_case("left");
    if !encrypt_op && !args[2].eq_ignore_ascii_case("right"){
        println!("Usage: file_encryptor <file|contents> <left|right> <iterations> <output_file>(optional)");
        return;
    }
    let input: Vec<u8> = fs::read(args[1].to_string()).unwrap_or(args[1].clone().into_bytes());
    let iterations: u8 = args[3].parse::<u8>().expect("Execpted an number");
    let output;
    if encrypt_op{
        output = encrypt(input, iterations);
    }else{
        output = decrypt(input, iterations);
    }
    if need_file_output{
        let mut file = OpenOptions::new().write(true).create(true).open(&args[4]).expect("Failed to create output file");
        file.write_all(&output).expect("Failed to write");
    }else{
        for c in output{
            print!("{}", c as char);
        }
        println!();
    }
    println!("Terminated successfully!");
}
