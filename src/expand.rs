use crate::decoder::decode;

pub(crate) fn write_expanded(){
    let data = decode();
    match data {
        Some(data)=> {
            let (mapping, mut vec) = data;
            //println!("{:?}", vec);
            //println!("{:?}", mapping);
            ////println!("{}",vec.len());
            let len = u64::from_be_bytes(vec[0..8].try_into().unwrap());
            //println!("{:?}", len);
            vec = vec[8..].to_owned();
            let mut bit_string = String::new();
            let mut text = String::new();
            let mut i: u64 = 0;
            for byte in vec{
                for j in [7,6,5,4,3,2,1,0]{
                    if i>=len{
                        break;
                    }
                    let bit = (byte >> j) & 1;
                    //println!("{}",bit);
                    if bit == 1{
                        bit_string.push('1');
                    }else{
                        bit_string.push('0');
                    }
                    if mapping.contains_key(&bit_string) {
                        //println!("{}", mapping[&bit_string]);
                        text.push(mapping.get(&bit_string).unwrap().parse().unwrap());
                        bit_string.clear();
                        i+=1;
                    }
                }
            }
            if let Err(e) = std::fs::write("./to_encode/expanded.txt", text) {
                eprintln!("Error writing expanded output to ./to_encode/expanded.txt:\n{:?}", e);
            }
        }
        None=>{
            eprintln!("Failed to decode Huffman mapping.");
        }
    }
}