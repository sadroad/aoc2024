use std::fs;
mod days;
use days::*;

pub trait Solution {
    fn solution(input: &str) -> (Option<i32>, Option<i32>);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please enter the day you want to run: ./aco2024 day");
        return;
    }
    let input = fs::read_to_string(format!("input/day{}", args[1])).unwrap();
    let (part1, part2) = match args[1].as_str() {
        "1" => Day1::solution(&input),
        "2" => Day2::solution(&input),
        "3" => Day3::solution(&input),
        _ => {
            eprintln!("Day {} not implemented yet", args[1]);
            return;
        }
    };
    if let Some(result) = part1 {
        println!("Part 1: {}", result);
    }
    if let Some(result) = part2 {
        println!("Part 2: {}", result);
    }
}
