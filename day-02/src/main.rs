use helper::{Cubes, Round};
use shared::read_input;

pub mod helper;

fn part_one(games: &Vec<Round>) -> u32 {
    let max = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    let res: u32 = games
        .iter()
        .filter(|g| {
            g.rounds
                .iter()
                .all(|r| r.blue <= max.blue && r.red <= max.red && r.green <= max.green)
        })
        .map(|g| g.id)
        .sum();

    res
}

fn part_two(games: &Vec<Round>) -> u32 {
    let res: u32 = games
        .iter()
        .map(|g| Cubes {
            red: g.rounds.iter().map(|r| r.red).max().unwrap(),
            green: g.rounds.iter().map(|r| r.green).max().unwrap(),
            blue: g.rounds.iter().map(|r| r.blue).max().unwrap(),
        })
        .map(|c| c.power())
        .sum();
    res
}

fn main() {
    let input: String = read_input!();

    let games = input.split('\n').map(Round::parse).collect();

    println!("Part one: {}", part_one(&games));
    println!("Part two: {}", part_two(&games));
}
