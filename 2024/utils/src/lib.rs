use std::env;
use std::fs;
use std::io;

pub fn read_from_args() -> io::Result<String> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    fs::read_to_string(file_path)
}

pub fn read_array_from_string(text: String) -> Vec<Vec<char>> {
    text.lines().map(
        |line| line.chars().collect::<Vec<char>>()
    ).collect()
}

pub fn read_array_of_numbers_from_string(text: String) -> Vec<Vec<u32>> {
    text.lines().map(
        |line| line.chars()
        .map(
            |c| if c == '.' {100} else {c.to_digit(10).unwrap()}
        )
        .collect::<Vec<u32>>()
    ).collect()
}