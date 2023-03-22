use std::str::FromStr;

pub fn solve(input: &Vec<String>) -> String {
    input
        .into_iter()
        .fold(0_i32, |mut acc: i32, line: &String| {
            let guide: Vec<&str> = line.split(" ").collect();
            let guide_move = guide[0].parse::<Move>().unwrap();
            let guide_outcome = guide[1].parse::<Outcome>().unwrap();

            let my_move = ((guide_move as i32) + (guide_outcome as i32) / 3 - 2).rem_euclid(3) + 1;
            let my_outcome = guide_outcome as i32;
            acc += my_move + my_outcome;
            acc
        })
        .to_string()
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("Not a known outcome".to_string()),
        }
    }
}
