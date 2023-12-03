use helper::parse;
use shared::read_input;

pub mod helper;

fn part_one(input: &str) -> u32 {
    parse(input)
        .parts
        .iter()
        .filter(|x| x.number)
        .map(|x| x.val)
        .sum::<u32>()
        .into()
}

fn part_two(input: &str) -> u32 {
    parse(input)
        .ratios
        .iter()
        .filter(|(_, vals)| vals.len() == 2)
        .map(|(_, vals)| vals[0] * vals[1])
        .sum::<u32>()
        .into()
}

fn main() {
    let input: String = read_input!();

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
