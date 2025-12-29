use crate::compress::write_compressed;
use crate::expand::write_expanded;
use std::io;

mod compress;
mod config;
mod decoder;
mod encoder;
mod expand;
mod file_io;
mod sturcture;

fn main() {
    let config = config::load();
    let mode = loop {
        let mut input = String::new();

        println!("Enter 'encode' to compress or 'decode' to get actual data:");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim().to_lowercase();

        match input.as_str() {
            "encode" | "decode" => break input,
            _ => println!("Invalid input. Please type 'encode' or 'decode'.\n"),
        }
    };

    println!("Selected mode: {}", mode);

    if mode == "encode" {
        println!("Starting compression...");
        write_compressed(&config);
    } else {
        println!("Starting decompression...");
        write_expanded(&config);
    }
}
