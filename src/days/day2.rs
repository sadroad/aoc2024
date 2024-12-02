use crate::Solution;

pub struct Day2 {}

impl Solution for Day2 {
    fn solution(input: &str) -> (Option<i32>, Option<i32>) {
        fn is_valid_sequence(nums: &[i32]) -> bool {
            let mut prior: Option<i32> = None;
            let mut diff_direction: Option<bool> = None;

            for num in nums {
                if let Some(prev) = prior {
                    let current_diff = num - prev;

                    if current_diff.abs() < 1 || current_diff.abs() > 3 {
                        return false;
                    }

                    match diff_direction {
                        None => diff_direction = Some(current_diff > 0),
                        Some(is_increasing) => {
                            if (current_diff > 0) != is_increasing {
                                return false;
                            }
                        }
                    }
                }
                prior = Some(*num);
            }
            true
        }

        let mut part1 = 0;
        let mut part2 = 0;

        for line in input.lines() {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            if is_valid_sequence(&nums) {
                part1 += 1;
                part2 += 1;
                continue;
            }

            for i in 0..nums.len() {
                let mut test_nums = nums.clone();
                test_nums.remove(i);
                if is_valid_sequence(&test_nums) {
                    part2 += 1;
                    break;
                }
            }
        }

        (Some(part1), Some(part2))
    }
}
