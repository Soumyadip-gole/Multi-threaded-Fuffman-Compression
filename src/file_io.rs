use crate::config::Config;
use std::fs;

pub fn read(config: &Config) -> Option<String> {
    let path = config.encode_input_dir.join("input.txt");
    let contents = fs::read_to_string(&path);
    println!("Reading file");
    match contents {
        Ok(contents) => {
            Some(contents)
        }
        Err(e) => {
            eprintln!("Error reading file {}:\n{:?}", path.display(), e);
            None
        }
    }
}

pub fn read_binary(config: &Config) -> Option<Vec<u8>> {
    let path = config.decode_input_dir.join("input.bin");
    let contents = fs::read(&path);
    match contents {
        Ok(contents) => {
            println!("Reading file");
            Some(contents)
        }
        Err(e) => {
            eprintln!("Error reading file {}:\n{:?}", path.display(), e);
            None
        }
    }
}
