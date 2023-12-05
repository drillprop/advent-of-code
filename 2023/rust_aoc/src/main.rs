mod day01;
mod day02;
mod utils;

fn main() {
    let result = day01::day01_a();
    println!("day1 a: {result}");
    let result = day01::day01_b();
    println!("day1 b: {result}");
    let result = day02::day02_a();
    println!("day2: {result}");
    let result = day02::day02_b();
    println!("day2: {result}");
}
