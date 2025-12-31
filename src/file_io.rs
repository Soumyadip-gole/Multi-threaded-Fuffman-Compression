use std::fs;

pub fn read(filename:String) -> Option<String> {
    let path = "to_encode/".to_string() + &filename;
    let contents = fs::read_to_string(&path);
    println!("Reading file {}", path);
    match contents {
        Ok(contents) => {
            Some(contents)
        }
        Err(_) => {
            eprintln!("Error reading file ");
            None
        }
    }
}

pub fn read_binary(filename:String) -> Option<Vec<u8>> {
    let path = "to_decode/".to_string() + &filename;
    let contents = fs::read(&path);
    match contents {
        Ok(contents) => {
            println!("Reading file: {}", path);
            Some(contents)
        }
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            None
        }
    }
}
