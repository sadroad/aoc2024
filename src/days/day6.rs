use crate::Solution;
use std::collections::HashSet;

pub struct Day6 {}

fn turn_right(direction: (i32, i32)) -> (i32, i32) {
    match direction {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => unreachable!(),
    }
}

impl Solution for Day6 {
    fn solution(input: &str) -> (Option<i32>, Option<i32>) {
        let part1 = solve_part1(input);
        let part2 = solve_part2(input);
        (Some(part1), Some(part2))
    }
}

fn solve_part1(input: &str) -> i32 {
    let mut obstacles = HashSet::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut guard_pos = (0, 0);
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstacles.insert((x as i32, y as i32));
                }
                '^' => {
                    guard_pos = (x as i32, y as i32);
                }
                _ => {}
            }
        }
    }

    let mut current_pos = guard_pos;
    let mut visited = HashSet::from([current_pos]);
    let mut direction = (0, -1);

    let mut next_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);

    while next_pos.0 >= 0 && next_pos.0 < width && next_pos.1 >= 0 && next_pos.1 < height {
        if obstacles.contains(&next_pos) {
            direction = turn_right(direction);
            next_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
        } else {
            current_pos = next_pos;
            visited.insert(current_pos);
            next_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
        }
    }

    visited.len() as i32
}

fn solve_part2(input: &str) -> i32 {
    let mut obstacles = HashSet::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut guard_pos = (0, 0);
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstacles.insert((x as i32, y as i32));
                }
                '^' => {
                    guard_pos = (x as i32, y as i32);
                }
                _ => {}
            }
        }
    }

    let mut current_pos = guard_pos;
    let mut direction = (0, -1);
    let mut next_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);

    let mut visited = HashSet::from([(current_pos, direction)]);
    let mut clear_tiles = HashSet::from([current_pos]);
    let mut extra_obstacles = HashSet::new();

    while next_pos.0 >= 0 && next_pos.0 < width && next_pos.1 >= 0 && next_pos.1 < height {
        if obstacles.contains(&next_pos) {
            direction = turn_right(direction);
            next_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
        } else {
            if !clear_tiles.contains(&next_pos) {
                let mut test_pos = current_pos;
                let mut test_dir = turn_right(direction);
                let mut test_visited = HashSet::new();

                let mut test_next = (test_pos.0 + test_dir.0, test_pos.1 + test_dir.1);

                while test_next.0 >= 0
                    && test_next.0 < width
                    && test_next.1 >= 0
                    && test_next.1 < height
                {
                    if obstacles.contains(&test_next) || test_next == next_pos {
                        test_dir = turn_right(test_dir);
                        test_next = (test_pos.0 + test_dir.0, test_pos.1 + test_dir.1);
                    } else if visited.contains(&(test_next, test_dir))
                        || test_visited.contains(&(test_next, test_dir))
                    {
                        extra_obstacles.insert(next_pos);
                        break;
                    } else {
                        test_pos = test_next;
                        test_visited.insert((test_pos, test_dir));
                        test_next = (test_pos.0 + test_dir.0, test_pos.1 + test_dir.1);
                    }
                }
            }

            current_pos = next_pos;
            visited.insert((current_pos, direction));
            clear_tiles.insert(current_pos);
            next_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
        }
    }

    extra_obstacles.len() as i32
}
