use crate::Solution;

pub struct Day3 {}

impl Solution for Day3 {
    fn solution(input: &str) -> (Option<i64>, Option<i64>) {
        let mut enabled = true;
        let mut part2 = 0;
        let mut part1 = 0;
        let mut all_matches: Vec<(usize, &str)> = vec![];
        all_matches.extend(input.match_indices("do()").map(|(i, _)| (i, "do")));
        all_matches.extend(input.match_indices("don't()").map(|(i, _)| (i, "don't")));
        all_matches.extend(input.match_indices("mul(").map(|(i, _)| (i, "mul")));
        all_matches.sort_by_key(|&(i, _)| i);
        for (start, op) in all_matches {
            let substr = &input[start..];
            match op {
                "do" => enabled = true,
                "don't" => enabled = false,
                "mul" => {
                    let mut chars = substr[4..].chars();
                    let mut num1 = 0;
                    for (digit_count, c) in chars.by_ref().enumerate() {
                        if c == ',' {
                            break;
                        }
                        if !c.is_ascii_digit() || digit_count >= 3 {
                            break;
                        }
                        num1 = num1 * 10 + (c as i64 - '0' as i64);
                    }
                    if num1 == 0 {
                        continue;
                    }

                    let mut num2 = 0;
                    for (digit_count, c) in chars.enumerate() {
                        if c == ')' {
                            if digit_count > 0 {
                                if enabled {
                                    part2 += num1 * num2;
                                }
                                part1 += num1 * num2;
                            }
                            break;
                        }
                        if !c.is_ascii_digit() || digit_count >= 3 {
                            break;
                        }
                        num2 = num2 * 10 + (c as i64 - '0' as i64);
                    }
                }
                _ => {}
            }
        }
        (Some(part1), Some(part2))
    }
}
