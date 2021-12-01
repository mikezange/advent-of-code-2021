use common::numbers;

fn main() {
    let input = &numbers("input.txt");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(input: &Vec<i32>) -> usize {
    input
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

fn part2(input: &Vec<i32>) -> usize {
    input
        .windows(3)
        .map(|window| -> i32 { window.iter().sum() })
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}
