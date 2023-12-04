use std::fs;

pub fn get_content(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    contents
}
