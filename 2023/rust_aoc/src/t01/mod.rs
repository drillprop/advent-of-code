use std::fs;

pub fn main() -> u32 {
    let contents =
        fs::read_to_string("./src/t01/input.txt").expect("Should have been able to read the file");

    let numbers: u32 = contents.lines().map(find_numbers).sum();

    numbers
}

fn find_numbers(s: &str) -> u32 {
    let parts = s.chars();
    let mut extracted_number = "".to_owned();

    for part in parts {
        if part.is_numeric() {
            extracted_number = extracted_number + &part.to_string();
        }
    }

    let first_number = extracted_number
        .to_owned()
        .chars()
        .next()
        .unwrap()
        .to_string();

    let last_number = extracted_number
        .to_owned()
        .chars()
        .last()
        .unwrap()
        .to_string();

    (first_number + &last_number).parse().unwrap()
}
