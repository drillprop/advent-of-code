use crate::utils;

pub fn a() -> u32 {
    let contents = utils::get_content("./src/day01/input.txt");
    let result: u32 = contents.lines().map(find_first_and_last_numbers).sum();
    result
}

pub fn b() -> u32 {
    let contents = utils::get_content("./src/day01/input.txt");
    let result: u32 = contents
        .lines()
        .map(find_first_and_last_numbers_spelled_out_with_letters)
        .sum();
    result
}

fn find_first_and_last_numbers_spelled_out_with_letters(s: &str) -> u32 {
    let valid_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let first_numeric_idx = s.find(char::is_numeric).unwrap();
    let last_numeric_idx = s.rfind(char::is_numeric).unwrap();

    let mut vector: Vec<(usize, String)> = Vec::new();

    valid_digits.iter().enumerate().for_each(|(_index, digit)| {
        let occurences: Vec<(usize, &str)> = s.match_indices(digit).collect();

        if occurences.len() > 0 {
            occurences
                .iter()
                .for_each(|(index, digit)| vector.push((*index, digit.to_string())))
        }
    });

    let mut new_str = s.to_owned();

    vector.sort_by(|a, b| a.0.cmp(&b.0));

    vector.last().iter().for_each(|x| {
        let idx = x.0;
        let lenght = x.1.len();
        if x.0 > last_numeric_idx {
            let index = valid_digits.iter().position(|&r| r == x.1).unwrap();

            new_str.replace_range(idx..idx + lenght, &(index + 1).to_string());
        }
    });

    vector.first().iter().for_each(|x| {
        if x.0 < first_numeric_idx {
            let index = valid_digits.iter().position(|&r| r == x.1).unwrap();
            new_str = new_str.replace(&x.1, &(index + 1).to_string());
        }
    });

    find_first_and_last_numbers(&new_str)
}

fn find_first_and_last_numbers(s: &str) -> u32 {
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
