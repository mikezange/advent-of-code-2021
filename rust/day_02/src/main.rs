use common::strings;

fn main() {
    let input = &strings("input.txt");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(input: &Vec<String>) -> i32 {
    let mut position = 0;
    let mut depth = 0;

    for line in input {
        let command: Vec<&str> = line.split(' ').collect();
        let value: i32 = command[1].parse().unwrap();

        match command[0] {
            "forward" => position += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => {}
        }
    }

    position * depth
}

fn part2(input: &Vec<String>) -> i32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input {
        let command: Vec<&str> = line.split(' ').collect();
        let value: i32 = command[1].parse().unwrap();

        match command[0] {
            "forward" => {
                position += value;
                depth += value * aim;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => {}
        }
    }

    position * depth
}
