use crate::compress::write_compressed;
use crate::expand::write_expanded;
use crate::path_read::reader;
use crate::thread_pool::process_files_parallel;
use std::io;
use std::time::Instant;

mod compress;
mod config;
mod decoder;
mod encoder;
mod expand;
mod file_io;
mod path_read;
mod sturcture;
mod thread_pool;

fn main() {
    let _config = config::load();
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
    let start = Instant::now();
    if mode == "encode" {
        println!("Encoding...");
        thread_pool::init_thread_pool(8);
        let files = reader("./to_encode".to_string()).unwrap();
        process_files_parallel(files, write_compressed);
    } else {
        println!("Decoding...");
        thread_pool::init_thread_pool(8);
        let files = reader("./to_decode".to_string()).unwrap();
        process_files_parallel(files, write_expanded);
    }

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}
