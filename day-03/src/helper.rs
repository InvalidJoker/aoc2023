use nd_vec::{vector, Vec2};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Result {
    pub parts: Vec<Part>,
    pub ratios: HashMap<Vec2<usize>, Vec<u32>>,
}

pub fn parse(input: &str) -> Result {
    let lines: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut symbol_positions = HashSet::new();
    let mut ratios = HashMap::new();
    let mut parts = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if !"0123456789.".contains(*c) {
                symbol_positions.insert(vector!(x, y));
            }
        }
    }

    for (y, line) in lines.iter().enumerate() {
        process_line(
            y,
            line,
            &mut symbol_positions,
            &mut parts,
            &mut ratios,
            &lines,
        );
    }

    Result { parts, ratios }
}

fn process_line(
    y: usize,
    line: &[char],
    symbol_positions: &mut HashSet<Vec2<usize>>,
    parts: &mut Vec<Part>,
    ratios: &mut HashMap<Vec2<usize>, Vec<u32>>,
    lines: &[Vec<char>],
) {
    let mut pos = None;

    let mut process_value_range = |pos: Option<usize>, x: usize| {
        if let Some(start) = pos {
            let end = x - 1;
            let val_str: String = lines[y][start..=end].iter().collect();
            if let Ok(val) = val_str.parse::<u32>() {
                let mut number = false;
                for nx in ((start as isize).saturating_sub(1))..=(end as isize + 1) {
                    for ny in ((y as isize).saturating_sub(1))..=(y as isize + 1) {
                        if nx < 0 || ny < 0 {
                            continue;
                        }
                        let [nx, ny] = [nx as usize, ny as usize];
                        let pos = vector!(nx, ny);
                        if symbol_positions.contains(&pos) && lines[ny][nx] == '*' {
                            ratios.entry(pos).or_insert(Vec::new()).push(val);
                        }
                        number |= symbol_positions.contains(&pos);
                    }
                }
                parts.push(Part { val, number });
            }
        }
    };

    let mut x = 0;
    while x < line.len() {
        if line[x].is_numeric() {
            if pos.is_none() {
                pos = Some(x);
            }
        } else {
            process_value_range(pos, x);
            pos = None;
        }
        x += 1;
    }
    process_value_range(pos, x);
}

#[derive(Debug)]
pub struct Part {
    pub val: u32,
    pub number: bool,
}
