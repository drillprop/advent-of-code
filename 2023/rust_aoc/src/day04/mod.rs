use crate::utils;

pub fn a() -> u32 {
    let contents = utils::get_content("./src/day04/input.txt");
    let sum: u32 = contents.lines().map(get_winining_numbers).sum();
    sum
}

pub fn b() -> u32 {
    // let contents = utils::get_content("./src/day04/input.txt");

    0
}

fn get_winining_numbers(s: &str) -> u32 {
    let sanitized_string = s
        .trim_start_matches("Card ")
        .splitn(2, ':')
        .nth(1)
        .unwrap_or("")
        .trim();

    let mut iter = sanitized_string.splitn(2, '|').map(|x| {
        x.trim()
            .split_whitespace()
            .filter_map(|num| num.parse::<u32>().ok())
            .collect::<Vec<u32>>()
    });

    let numbers_tuple: (Vec<u32>, Vec<u32>) = match (iter.next(), iter.next()) {
        (Some(first_vec), Some(second_vec)) => (first_vec, second_vec),
        _ => (Vec::new(), Vec::new()),
    };

    let duplicates: u32 = numbers_tuple.0.iter().fold(0, |acc, winning_number| {
        if numbers_tuple
            .1
            .iter()
            .any(|your_number| your_number == winning_number)
        {
            acc + 1
        } else {
            acc
        }
    });

    if duplicates >= 2 {
        2u32.pow(duplicates - 1)
    } else {
        duplicates
    }
}
