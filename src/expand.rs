use crate::decoder::decode;
use std::fs;

pub(crate) fn write_expanded(filename: String) {
    println!("Writing expanded code for: {}", filename);
    let data = decode(filename.clone()).unwrap();
    let (mapping, mut vec) = data;
    let len = u64::from_be_bytes(vec[0..8].try_into().unwrap());
    //println!("{:?}", len);

    vec = vec[8..].to_owned();
    let mut bit_string = String::new();
    let mut text = String::new();
    let mut i: u64 = 0;

    for byte in vec {
        for j in [7, 6, 5, 4, 3, 2, 1, 0] {
            if i >= len {
                break;
            }
            let bit = (byte >> j) & 1;
            //println!("{}",bit);
            if bit == 1 {
                bit_string.push('1');
            } else {
                bit_string.push('0');
            }
            if mapping.contains_key(&bit_string) {
                //println!("{}", mapping[&bit_string]);
                text.push_str(mapping.get(&bit_string).unwrap());
                bit_string.clear();
                i += 1;
            }
        }
    }
    fs::create_dir_all("./decoded_output").unwrap();

    let out_path = format!("./decoded_output/{}_decoded.txt", filename);
    if let Err(e) = std::fs::write(&out_path, &text) {
        eprintln!("Error writing expanded output: {}", e);
    } else {
        println!("Decoded output written to: {}", out_path);
    }
}
