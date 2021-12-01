use std::{fs, path::Path};

pub fn numbers(filename: &str) -> Vec<i32> {
    let input = read_file(filename);

    input
        .trim()
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn read_file(filename: impl AsRef<Path>) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}
