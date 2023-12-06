use crate::utils;

pub fn a() -> u32 {
    let contents = utils::get_content("./src/day03/input.txt");
    let characters: Vec<char> = vec!['*', '#', '+', '$', '%', '/', '&', '=', '-', '@'];
    let symbol_coordinates = get_symbols_coordinates(&contents, &characters);
    let number_coordinates = get_digits_coordinates(&contents);

    sum_adjacent_values(symbol_coordinates, number_coordinates)
}

pub fn b() -> u32 {
    let contents = utils::get_content("./src/day03/input.txt");
    let characters: Vec<char> = vec!['*'];
    let symbol_coordinates = get_symbols_coordinates(&contents, &characters);
    let number_coordinates = get_digits_coordinates(&contents);

    let result = sum_all_gears(symbol_coordinates, number_coordinates);

    result
}

#[derive(Debug, Clone)]
struct SymbolCoordinates {
    x: usize,
    y: usize,
}

fn get_symbols_coordinates(str: &str, pattern: &Vec<char>) -> Vec<SymbolCoordinates> {
    let symbol_coordinates: Vec<_> = str
        .lines()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            line.chars().enumerate().filter_map(move |(row_idx, char)| {
                if pattern.contains(&char) {
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
    let number_coordinates: Vec<NumberCoordinates> =
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
            .fold(
                Vec::<NumberCoordinates>::new(),
                |mut acc, NumberCoordinates { y, x, value }| {
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
                },
            )
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

#[derive(Debug, Clone)]
struct Gear {
    value: u32,
    symbol_coordinates: SymbolCoordinates,
}

fn sum_all_gears(
    symbol_coordinates: Vec<SymbolCoordinates>,
    number_coordinates: Vec<NumberCoordinates>,
) -> u32 {
    let gears_vector: Vec<_> = number_coordinates
        .iter()
        .filter_map(|NumberCoordinates { value, x, y }| {
            let x_left_match = x.max(&1) - 1;
            let x_right_match = x + value.to_string().len() + 1;
            let x_range = x_left_match..x_right_match;
            let y_top_match = y.max(&1) - 1;
            let y_bottom_match = y + 2;
            let y_range = y_top_match..y_bottom_match;

            let symbol_coordinates_that_match = symbol_coordinates
                .iter()
                .find(|SymbolCoordinates { x, y }| x_range.contains(&x) && y_range.contains(y));

            match symbol_coordinates_that_match {
                Some(symbols) => {
                    let gear = Gear {
                        value: *value,
                        symbol_coordinates: symbols.clone(),
                    };
                    Some(gear)
                }
                None => None,
            }
        })
        .collect();

    let unduplicated_gears: Vec<Gear> = gears_vector
        .iter()
        .filter(|gear| {
            let match_coordinates: Vec<_> = gears_vector
                .iter()
                .filter(|gearb| {
                    gear.symbol_coordinates.x == gearb.symbol_coordinates.x
                        && gear.symbol_coordinates.y == gearb.symbol_coordinates.y
                })
                .collect();
            match_coordinates.len() == 2
        })
        .fold(Vec::new(), |mut acc, gear| {
            let exist = acc.iter().any(|existing_gear| {
                existing_gear.symbol_coordinates.x == gear.symbol_coordinates.x
                    && existing_gear.symbol_coordinates.y == gear.symbol_coordinates.y
            });

            if exist {
                if let Some(prev) = acc.iter_mut().find(|existing_gear| {
                    gear.symbol_coordinates.x == existing_gear.symbol_coordinates.x
                        && gear.symbol_coordinates.y == existing_gear.symbol_coordinates.y
                }) {
                    prev.value = prev.value * gear.value;
                }
            } else {
                acc.push(gear.clone());
            }
            acc
        });

    let sum: u32 = unduplicated_gears.iter().map(|x| x.value).sum();

    sum
}
