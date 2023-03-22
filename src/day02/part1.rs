use std::str::FromStr;

pub fn solve(input: &Vec<String>) -> String {
    input
        .into_iter()
        .map(|line| {
            line.split(" ")
                .map(|m| m.parse::<Move>().unwrap())
                .collect()
        })
        .fold(0_u32, |mut acc: u32, moves: Vec<Move>| {
            match moves[0].partial_cmp(&moves[1]) {
                Some(std::cmp::Ordering::Equal) => {
                    acc += 3 + moves[1] as u32;
                }
                Some(std::cmp::Ordering::Greater) => {
                    acc += moves[1] as u32;
                }
                Some(std::cmp::Ordering::Less) => {
                    acc += 6 + moves[1] as u32;
                }
                None => {
                    panic!("Moves should be comparable")
                }
            }
            acc
        })
        .to_string()
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Move::Rock && other == &Move::Scissors {
            Some(std::cmp::Ordering::Greater)
        } else if self == &Move::Scissors && other == &Move::Rock {
            Some(std::cmp::Ordering::Less)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}
