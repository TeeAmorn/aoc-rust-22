use std::fs;
use std::time;

mod day01;

#[derive(Debug)]
pub struct Args {
    pub day: u8,
    pub part: u8,
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        if args.len() <= 1 {
            return Err("Not enough arguments");
        } else if args.len() >= 4 {
            return Err("Too many arguments");
        }

        let day: u8 = match args[1].parse() {
            Ok(n) => {
                if n < 1 || n > 25 {
                    return Err("1st argument must be a number between 1 and 25");
                }
                n
            }
            Err(_) => {
                return Err("1st argument must be a number between 1 and 25");
            }
        };

        let mut part: u8 = 1;
        if args.len() == 3 {
            part = match args[2].parse() {
                Ok(n) => {
                    if n < 1 || n > 2 {
                        return Err("2nd argument must be 1 or 2");
                    }
                    n
                }
                Err(_) => {
                    return Err("2nd argument must be 1 or 2");
                }
            };
        }

        Ok(Args { day, part })
    }
}

pub fn parse_input(args: &Args) -> Result<Vec<String>, String> {
    let file_path = format!("src/day{:02}/input.txt", args.day);
    let contents = fs::read_to_string(&file_path);

    if let Ok(input) = contents {
        Ok(input
            .lines()
            .map(|line| String::from(line.trim()))
            .collect())
    } else {
        Err(format!("Cannot parse input file at {}", &file_path))
    }
}

#[derive(Debug)]
pub struct Output {
    pub time: u128,
    pub value: String,
}

pub fn solve(args: &Args, input: &Vec<String>) -> Result<Output, &'static str> {
    let start = time::SystemTime::now();

    let output = get_solver(&args)?(input);
    if let Ok(elapsed) = start.elapsed() {
        Ok(Output {
            time: elapsed.as_millis(),
            value: output,
        })
    } else {
        Err("Cannot compute elapsed time")
    }
}

fn get_solver(args: &Args) -> Result<fn(&Vec<String>) -> String, &'static str> {
    let id = format!("{:02}", args.day) + format!("{:02}", args.part).as_str();
    match id.as_str() {
        "0101" => Ok(crate::day01::part1::solve),
        "0102" => Ok(crate::day01::part2::solve),
        _ => Err("Cannot find method"),
    }
}
