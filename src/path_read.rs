use std::fs;
use std::io;

pub fn reader(path: String) -> io::Result<Vec<String>> {
    let mut file_names = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                file_names.push(file_name.to_str().unwrap().to_string());
            }
        }
    }

    Ok(file_names)
}
