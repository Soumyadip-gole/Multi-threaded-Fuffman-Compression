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
mod structure;
mod thread_pool;

fn main() {
    let config = config::load();
    println!("Enter number of threads:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let threads: usize = input.trim().parse().expect("Please enter a valid number");
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
        thread_pool::init_thread_pool(threads);
        let files = reader(config.encode_input_dir.to_string_lossy().into_owned()).unwrap();
        process_files_parallel(files, |file| {
            write_compressed(file, &config.encode_input_dir, &config.encoded_output_dir);
        });
    } else {
        println!("Decoding...");
        thread_pool::init_thread_pool(threads);
        let files = reader(config.decode_input_dir.to_string_lossy().into_owned()).unwrap();
        process_files_parallel(files, |file| {
            write_expanded(file, &config.decode_input_dir, &config.decoded_output_dir);
        });
    }

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}
