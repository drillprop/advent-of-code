use crate::utils;

pub fn a() -> i32 {
    let contents = utils::get_content("./src/day02/input.txt");

    let result: i32 = contents.lines().map(count_if_possible).sum();
    result
}

pub fn b() -> i32 {
    let contents = utils::get_content("./src/day02/input.txt");

    let result: i32 = contents.lines().map(power_games_with_fewest_games).sum();
    result
}

fn power_games_with_fewest_games(s: &str) -> i32 {
    let game = s.split_once(&[':']).map_or(s, |(_game_id, game)| game);

    let cubes_vector: Vec<Cubes> = game
        .split_terminator([';'])
        .map(|str| extract_cubes(str))
        .collect();

    let reds = cubes_vector.iter().map(|cube| cube.red).max().unwrap_or(1);
    let green = cubes_vector
        .iter()
        .map(|cube| cube.green)
        .max()
        .unwrap_or(1);
    let blue = cubes_vector.iter().map(|cube| cube.blue).max().unwrap_or(1);

    let result = reds * green * blue;
    result
}

fn count_if_possible(s: &str) -> i32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let segments: Vec<_> = s.split_terminator(&[':', ';'][..]).collect();

    let mut game_id = 0;
    segments.iter().enumerate().for_each(|(idx, string)| {
        if idx == 0 {
            game_id = split_numbers(string);
        }
        if idx > 0 {
            let cubes = extract_cubes(string);

            if cubes.green > max_green || cubes.blue > max_blue || cubes.red > max_red {
                game_id = 0;
            };
        }
    });
    game_id
}

#[derive(Debug)]
struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

fn extract_cubes(s: &str) -> Cubes {
    let mut cube = Cubes {
        red: 0,
        green: 0,
        blue: 0,
    };

    s.split(",").for_each(|x| {
        if x.contains("red") {
            cube.red = split_numbers(x)
        }
        if x.contains("green") {
            cube.green = split_numbers(x)
        }
        if x.contains("blue") {
            cube.blue = split_numbers(x)
        }
    });

    cube
}

fn split_numbers(string: &str) -> i32 {
    let digits = string
        .trim()
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<String>();

    let number: i32 = digits.parse().unwrap_or(0);

    number
}
