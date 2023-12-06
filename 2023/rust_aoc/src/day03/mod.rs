use crate::utils;

pub fn a() -> u32 {
    let contents = utils::get_content("./src/day03/input.txt");
    let symbol_coordinates = get_symbols_coordinates(&contents);
    let number_coordinates = get_digits_coordinates(&contents);

    sum_adjacent_values(symbol_coordinates, number_coordinates)
}

#[derive(Debug)]
struct SymbolCoordinates {
    x: usize,
    y: usize,
}

fn get_symbols_coordinates(str: &str) -> Vec<SymbolCoordinates> {
    let symbol_coordinates: Vec<_> = str
        .lines()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            let characters: Vec<char> = vec!['*', '#', '+', '$', '%', '/', '&', '=', '-', '@'];

            line.chars().enumerate().filter_map(move |(row_idx, char)| {
                if characters.contains(&char) {
                    Some(SymbolCoordinates {
                        x: row_idx,
                        y: line_idx,
                    })
                } else {
                    None
                }
            })
        })
        .collect();

    symbol_coordinates
}

#[derive(Clone, Debug)]
struct NumberCoordinates {
    value: u32,
    y: usize,
    x: usize,
}

fn get_digits_coordinates(str: &str) -> Vec<NumberCoordinates> {
    let new_vec: Vec<NumberCoordinates> = Vec::new();

    let number_coordinates: Vec<_> =
        str.lines()
            .enumerate()
            .flat_map(|(line_idx, line)| {
                let position_and_value: Vec<_> =
                    line.chars()
                        .enumerate()
                        .filter_map(|(row_idx, char)| {
                            char.to_string().parse::<u32>().ok().map(|parsed_value| {
                                NumberCoordinates {
                                    y: line_idx,
                                    x: row_idx,
                                    value: parsed_value,
                                }
                            })
                        })
                        .collect();
                position_and_value
            })
            .fold(new_vec, |mut acc, NumberCoordinates { y, x, value }| {
                if let Some(prev) = acc.iter().rev().next() {
                    if prev.x + 1 == x && y == prev.y {
                        let new_value = prev.value.to_string() + &value.to_string();
                        let mut new_acc = acc.clone();

                        new_acc.pop();
                        new_acc.push(NumberCoordinates {
                            y: prev.y,
                            x: prev.x + 1,
                            value: new_value.parse().unwrap_or(0),
                        });
                        acc = new_acc;
                    } else {
                        acc.push(NumberCoordinates { value, y, x });
                    }
                } else {
                    acc.push(NumberCoordinates { x, y, value });
                }

                acc
            })
            .iter()
            .map(|NumberCoordinates { x, y, value }| NumberCoordinates {
                x: x - (value.to_string().len() - 1),
                y: *y,
                value: *value,
            })
            .collect();

    number_coordinates
}

fn sum_adjacent_values(
    symbol_coordinates: Vec<SymbolCoordinates>,
    number_coordinates: Vec<NumberCoordinates>,
) -> u32 {
    let numbers_vec: Vec<u32> = number_coordinates
        .iter()
        .filter_map(|NumberCoordinates { value, x, y }| {
            let x_left_match = x.max(&1) - 1;
            let x_right_match = x + value.to_string().len() + 1;
            let x_range = x_left_match..x_right_match;
            let y_top_match = y.max(&1) - 1;
            let y_bottom_match = y + 2;
            let y_range = y_top_match..y_bottom_match;

            let has_symbols = symbol_coordinates
                .iter()
                .any(|SymbolCoordinates { x, y }| x_range.contains(&x) && y_range.contains(y));

            if has_symbols {
                Some(*value)
            } else {
                None
            }
        })
        .collect();

    numbers_vec.iter().sum()
}

pub fn b() -> i32 {
    // let contents = utils::get_content("./src/day03/input.txt");

    32
}
