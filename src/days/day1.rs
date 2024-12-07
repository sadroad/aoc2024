use crate::Solution;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub struct Day1 {}

impl Solution for Day1 {
    fn solution(input: &str) -> (Option<i64>, Option<i64>) {
        let mut heap1 = BinaryHeap::new();
        let mut heap2 = BinaryHeap::new();
        let mut count = HashMap::new();
        input
            .lines()
            .map(|x| x.split_whitespace())
            .for_each(|mut x| {
                heap1.push(x.next().unwrap().parse::<i64>().unwrap());
                let right_val = x.next().unwrap().parse::<i64>().unwrap();
                count.insert(right_val, count.get(&right_val).unwrap_or(&0) + 1);
                heap2.push(right_val);
            });
        let mut total_distance = 0;
        let mut similarity_score = 0;
        while let (Some(x), Some(y)) = (heap1.pop(), heap2.pop()) {
            total_distance += (x - y).abs();
            let right_times = count.get(&x).unwrap_or(&0);
            similarity_score += x * right_times;
        }
        (Some(total_distance), Some(similarity_score))
    }
}
