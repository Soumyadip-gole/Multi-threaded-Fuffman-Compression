use std::fs;

pub fn read()->Option<String>{
    let contents = fs::read_to_string("./to_encode/test.txt");
    match contents {
        Ok(contents) => {
            //println!("{}", contents);
            Some(contents)
        }
        Err(e) => {
            eprintln!("Error reading file ./to_encode/test.txt:\n{:?}", e);
            None
        }
    }
}

pub fn read_binary()->Option<Vec<u8>>{
    let contents = fs::read("./to_decode/output.bin");
    match contents {
        Ok(contents) => {
            //println!("{:?}", contents);
            Some(contents)
        }
        Err(e) => {
            eprintln!("Error reading file ./to_decode/output.bin:\n{:?}", e);
            None
        }
    }
}