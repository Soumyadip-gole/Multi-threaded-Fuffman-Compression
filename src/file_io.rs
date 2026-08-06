use std::fs;
use std::path::Path;

pub fn read(input_dir: &Path, filename: &str) -> Option<String> {
    let path = input_dir.join(filename);
    let contents = fs::read_to_string(&path);
    println!("Reading file {}", path.display());
    match contents {
        Ok(contents) => Some(contents),
        Err(_) => {
            eprintln!("Error reading file ");
            None
        }
    }
}

pub fn read_binary(input_dir: &Path, filename: &str) -> Option<Vec<u8>> {
    let path = input_dir.join(filename);
    let contents = fs::read(&path);
    match contents {
        Ok(contents) => {
            println!("Reading file: {}", path.display());
            Some(contents)
        }
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path.display(), e);
            None
        }
    }
}
