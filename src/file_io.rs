use std::fs;

pub fn read()->Option<String>{
    let contents = fs::read_to_string("./to_encode/test.txt");
    match contents {
        Ok(contents) => {
            println!("{}", contents);
            Some(contents)
        }
        Err(e) => {
            println!("Error reading file:  \n {:?}", e);
            None
        }
    }
}