use colored::Colorize;
use std::env;
use std::process;

use aoc::{parse_input, solve, Args};

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = Args::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    let input = parse_input(&args).unwrap_or_else(|err| {
        println!("Problem parsing input file: {err}");
        process::exit(1)
    });

    let output = solve(&args, &input).unwrap_or_else(|err| {
        println!("Problem solving problem: {err}");
        process::exit(1)
    });

    println!(
        "{} Day {} Part {} in {}ms: {}",
        "Solved".bold().green(),
        args.day.to_string().bold().green(),
        args.part.to_string().bold().green(),
        output.time.to_string().bold().blue(),
        output.value.bold().yellow(),
    )
}
