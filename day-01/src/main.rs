use helper::get_digits;
use shared::read_input;

pub mod helper;

fn part_one(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        sum += first * 10 + last;
    }

    sum
}

fn part_two(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let digits = get_digits(line);
        sum += digits[0] * 10 + digits[1];
    }

    sum
}

fn main() {
    let input = read_input!();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
