use common::strings;

fn main() {
    let input = &strings("input.txt");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(input: &Vec<String>) -> i32 {
    let mut bits = String::new();
    for i in 0..input[0].len() {
        bits += &get_most_common_bit(input, i).to_string();
    }

    let gamma = i32::from_str_radix(&bits, 2).unwrap();

    //inverse of gamma
    let epsilon = !gamma & 0b111111111111;

    gamma * epsilon
}

fn part2(input: &Vec<String>) -> i32 {
    let o2_rating = get_rating(input, true);
    let co2_rating = get_rating(input, false);
    o2_rating * co2_rating
}

fn get_rating(input: &Vec<String>, is_o2: bool) -> i32 {
    let mut input_clone = input.clone();
    for i in 0..12 {
        let bit = get_most_common_bit(&input_clone, i);
        input_clone = input_clone
            .iter()
            .cloned()
            .filter(|line| {
                (if is_o2 { bit } else { !bit & 1 })
                    == line.chars().nth(i).unwrap().to_digit(2).unwrap() as i32
            })
            .collect();

        if input_clone.len() == 1 {
            break;
        }
    }
    return i32::from_str_radix(input_clone.first().unwrap(), 2).unwrap();
}

fn get_most_common_bit(input: &Vec<String>, i: usize) -> i32 {
    let mut one_count = 0;

    for line in input {
        if line.chars().nth(i).unwrap() == '1' {
            one_count += 1;
        } else {
            one_count -= 1;
        }
    }

    (one_count >= 0) as i32
}
