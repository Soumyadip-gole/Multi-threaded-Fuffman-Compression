use crate::file_io::read_binary;
use std::collections::HashMap;

pub fn decode() -> Option<(HashMap<String, String>, Vec<u8>)> {
    let contents = read_binary();
    match contents {
        Some(bytes) => {
            let mut map: HashMap<String, String> = HashMap::new();
            let mut x = 4;
            let len = u32::from_be_bytes(bytes[0..x].try_into().unwrap());
            for _ in 0..len {
                let key_bytes = bytes[x];
                x += 1;
                let key = String::from_utf8(bytes[x..x + (key_bytes as usize)].try_into().unwrap());
                x += key_bytes as usize;
                let code_bytes = bytes[x];
                x += 1;
                let code = String::from_utf8(bytes[x..x + (code_bytes as usize)].try_into().unwrap());
                x += code_bytes as usize;
                map.insert(code.unwrap(), key.unwrap());
            }
            //println!("{:?}",map);
            return Some((map, bytes[x..].to_owned()));
        }
        None => {
            eprintln!("No content read from binary file.");
            return None;
        }
    }
}