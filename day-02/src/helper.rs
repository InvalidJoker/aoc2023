#[derive(Default, Debug)]
pub struct Cubes {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

impl Cubes {
    pub fn parse(raw: &str) -> Self {
        let mut round = Self::default();

        for cube in raw.split(',').map(str::trim) {
            let (n, color) = cube.split_once(' ').unwrap();
            let n = n.parse().unwrap();
            match color {
                "blue" => round.blue = n,
                "green" => round.green = n,
                "red" => round.red = n,
                _ => panic!("unexpected color: {color}"),
            }
        }

        round
    }

    pub fn power(&self) -> u32 {
        self.blue * self.green * self.red
    }
}

#[derive(Debug)]
pub struct Round {
    pub id: u32,
    pub rounds: Vec<Cubes>,
}

impl Round {
    pub fn parse(line: &str) -> Self {
        let idx = line.chars().position(|c| c == ':').unwrap();
        let id = line[5..idx].parse().unwrap();

        let rounds = line[idx + 1..].split(';').map(Cubes::parse).collect();

        Self { id, rounds }
    }
}
