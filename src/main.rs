mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let file_load_error = "Could not load input data";

    println!("=== Day 01 ===");
    println!(
        "Total distance is: {}",
        day01::problem1::run(load_input_file("src/day01/input.txt").expect(&file_load_error))
    );
    println!(
        "Similarity score is: {}",
        day01::problem2::run(load_input_file("src/day01/input.txt").expect(&file_load_error))
    );

    println!("=== Day 02 ===");
    println!(
        "Number of safe reports: {}",
        day02::problem1::run(load_input_file("src/day02/input.txt").expect(&file_load_error))
    );
    println!(
        "Number of safe reports after dampening: {}",
        day02::problem2::run(load_input_file("src/day02/input.txt").expect(&file_load_error))
    );

    println!("=== Day 03 ===");
    println!(
        "Result of all multiplications: {}",
        day03::problem1::run(load_input_file("src/day03/input.txt").expect(&file_load_error))
    );
    println!(
        "Result of all multiplications after adding 'logic': {}",
        day03::problem2::run(load_input_file("src/day03/input.txt").expect(&file_load_error))
    );

    println!("=== Day 04 ===");
    println!(
        "XMAS count: {}",
        day04::problem1::run(load_input_file("src/day04/input.txt").expect(&file_load_error))
    );
    println!(
        "MAS X count: {}",
        day04::problem2::run(load_input_file("src/day04/input.txt").expect(&file_load_error))
    );

    println!("=== Day 05 ===");
    println!(
        "Middle page energy: {}",
        day05::problem1::run(load_input_file("src/day05/input.txt").expect(&file_load_error))
    );
    println!(
        "Middle sorted page energy: {}",
        day05::problem2::run(load_input_file("src/day05/input.txt").expect(&file_load_error))
    );
}

fn load_input_file(file_path: &str) -> Result<impl Iterator<Item = String>, io::Error> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().filter_map(|line| line.ok()))
}
