use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs;
mod days;

pub trait Solution {
    fn solution(input: &str) -> (Option<i32>, Option<i32>);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please enter the day you want to run: ./aco2024 day");
        return;
    }
    let mut heap1 = BinaryHeap::new();
    let mut heap2 = BinaryHeap::new();
    let input = fs::read_to_string(format!("input/day{}", args[1])).unwrap();
    let mut count = HashMap::new();
    input
        .lines()
        .map(|x| x.split_whitespace())
        .for_each(|mut x| {
            heap1.push(x.next().unwrap().parse::<u32>().unwrap());
            let right_val = x.next().unwrap().parse::<u32>().unwrap();
            count.insert(right_val, count.get(&right_val).unwrap_or(&0) + 1);
            heap2.push(right_val);
        });
    let mut total_distance = 0;
    let mut similarity_score = 0;
    while let (Some(x), Some(y)) = (heap1.pop(), heap2.pop()) {
        total_distance += x.abs_diff(y);
        let right_times = count.get(&x).unwrap_or(&0);
        similarity_score += x * right_times;
    }
    println!("Part 1: {}", total_distance);
    println!("Part 2: {}", similarity_score);
}
