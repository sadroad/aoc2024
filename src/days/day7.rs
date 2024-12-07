use crate::Solution;
pub struct Day7 {}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenation,
}

fn can_make_target(numbers: &[i64], target: i64) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == target;
    }

    let mut operators: Vec<Operator> = Vec::new();
    let max_combinations = numbers.len() - 1;

    for _ in 0..max_combinations {
        operators.push(Operator::Add);
    }

    loop {
        let mut current = numbers[0];

        for (i, &operator) in operators.iter().enumerate() {
            match operator {
                Operator::Add => current += numbers[i + 1],
                Operator::Multiply => current *= numbers[i + 1],
                Operator::Concatenation => {
                    current = format!("{}{}", current, numbers[i + 1]).parse().unwrap()
                }
            }
        }

        if current == target {
            return true;
        }

        let mut done = true;
        for op in operators.iter_mut() {
            match op {
                Operator::Add => {
                    *op = Operator::Multiply;
                    done = false;
                    break;
                }
                Operator::Multiply => {
                    *op = Operator::Concatenation;
                    done = false;
                    break;
                }
                Operator::Concatenation => {
                    *op = Operator::Add;
                }
            }
        }
        if done {
            break;
        }
    }
    false
}

impl Solution for Day7 {
    fn solution(input: &str) -> (Option<i64>, Option<i64>) {
        let mut sum = 0;

        for line in input.lines() {
            let (target_str, nums_str) = line.split_once(":").unwrap();
            let target = target_str.trim().parse::<i64>().unwrap();

            let numbers: Vec<i64> = nums_str
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();

            if can_make_target(&numbers, target) {
                sum += target;
            }
        }

        (Some(sum), None)
    }
}
