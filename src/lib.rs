#![feature(iter_array_chunks)]

use std::fs;
use std::time;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

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
    println!("{}", file_path);
    let contents = fs::read_to_string(&file_path);

    if let Ok(input) = contents {
        Ok(input.lines().map(|line| String::from(line)).collect())
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
        "0201" => Ok(crate::day02::part1::solve),
        "0202" => Ok(crate::day02::part2::solve),
        "0301" => Ok(crate::day03::part1::solve),
        "0302" => Ok(crate::day03::part2::solve),
        "0401" => Ok(crate::day04::part1::solve),
        "0402" => Ok(crate::day04::part2::solve),
        "0501" => Ok(crate::day05::part1::solve),
        "0502" => Ok(crate::day05::part2::solve),
        "0601" => Ok(crate::day06::part1::solve),
        "0602" => Ok(crate::day06::part2::solve),
        "0701" => Ok(crate::day07::part1::solve),
        "0702" => Ok(crate::day07::part2::solve),
        "0801" => Ok(crate::day08::part1::solve),
        "0802" => Ok(crate::day08::part2::solve),
        "0901" => Ok(crate::day09::part1::solve),
        "0902" => Ok(crate::day09::part2::solve),
        "1001" => Ok(crate::day10::part1::solve),
        "1002" => Ok(crate::day10::part2::solve),
        "1101" => Ok(crate::day11::part1::solve),
        "1102" => Ok(crate::day11::part2::solve),
        "1201" => Ok(crate::day12::part1::solve),
        "1202" => Ok(crate::day12::part2::solve),
        _ => Err("Cannot find method"),
    }
}
