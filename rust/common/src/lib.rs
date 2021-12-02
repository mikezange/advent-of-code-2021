use std::{fs, path::Path};

pub fn numbers(filename: &str) -> Vec<i32> {
    let input = read_file(filename);

    input
        .trim()
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

pub fn strings(filename: &str) -> Vec<String> {
    let input = read_file(filename);
    input.trim().lines().map(String::from).collect()
}

fn read_file(filename: impl AsRef<Path>) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}
