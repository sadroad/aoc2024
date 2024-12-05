use crate::Solution;

pub struct Day4 {}

impl Solution for Day4 {
    fn solution(input: &str) -> (Option<i32>, Option<i32>) {
        let mut total_xmas = 0;
        total_xmas += find_forward(input);
        total_xmas += find_backward(input);
        total_xmas += find_vertical(input);
        total_xmas += find_vertical_backwards(input);
        total_xmas += find_diagonal_right(input);
        total_xmas += find_diagonal_left(input);
        total_xmas += find_diagonal_right_backward(input);
        total_xmas += find_diagonal_left_backward(input);
        (Some(total_xmas), Some(find_x_mas(input)))
    }
}

#[derive(Clone, Copy)]
enum TopPairs {
    MM,
    MS,
    SM,
    SS,
}

fn find_x_mas(input: &str) -> i32 {
    let line_length = input.lines().next().unwrap().len();
    let lines: Vec<_> = input.lines().collect();
    let mut instances = 0;
    for (i, line) in lines.iter().enumerate() {
        if i + 3 > lines.len() {
            break;
        }
        let top: Vec<_> = line
            .match_indices(['S', 'M'])
            .filter_map(|x| {
                if x.0 + 3 > line_length {
                    return None;
                }
                let pair = &line[x.0 + 2..x.0 + 2 + 1];
                match x.1 {
                    "M" if pair == "M" => Some((x.0, TopPairs::MM)),
                    "M" if pair == "S" => Some((x.0, TopPairs::MS)),
                    "S" if pair == "M" => Some((x.0, TopPairs::SM)),
                    "S" if pair == "S" => Some((x.0, TopPairs::SS)),
                    _ => None,
                }
            })
            .collect();
        if top.is_empty() {
            continue;
        }
        let middle_line = lines.get(i + 1).unwrap();
        let middle: Vec<_> = top
            .iter()
            .filter_map(|x| {
                if &middle_line[x.0 + 1..x.0 + 1 + 1] == "A" {
                    return Some((x.0 + 1, x.1));
                }
                None
            })
            .collect();
        if middle.is_empty() {
            continue;
        }
        let bottom_line = lines.get(i + 2).unwrap();
        let bottom: Vec<_> = middle
            .iter()
            .filter(|x| match x.1 {
                TopPairs::MM => {
                    &bottom_line[x.0 - 1..x.0 - 1 + 1] == "S"
                        && &bottom_line[x.0 + 1..x.0 + 1 + 1] == "S"
                }
                TopPairs::MS => {
                    &bottom_line[x.0 - 1..x.0 - 1 + 1] == "M"
                        && &bottom_line[x.0 + 1..x.0 + 1 + 1] == "S"
                }
                TopPairs::SM => {
                    &bottom_line[x.0 - 1..x.0 - 1 + 1] == "S"
                        && &bottom_line[x.0 + 1..x.0 + 1 + 1] == "M"
                }
                TopPairs::SS => {
                    &bottom_line[x.0 - 1..x.0 - 1 + 1] == "M"
                        && &bottom_line[x.0 + 1..x.0 + 1 + 1] == "M"
                }
            })
            .collect();
        instances += bottom.len() as i32;
    }
    instances
}

fn find_forward(input: &str) -> i32 {
    input
        .lines()
        .map(|x| x.matches("XMAS").count() as i32)
        .sum()
}

fn find_backward(input: &str) -> i32 {
    input
        .lines()
        .map(|x| x.matches("SAMX").count() as i32)
        .sum()
}

fn find_vertical(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    let mut instances = 0;
    for (i, line) in lines.iter().enumerate() {
        if i + 4 > lines.len() {
            break;
        }
        let x_s: Vec<_> = line.match_indices('X').map(|x| x.0).collect();
        if x_s.is_empty() {
            continue;
        }
        let m_s: Vec<_> = lines
            .get(i + 1)
            .unwrap()
            .match_indices('M')
            .filter_map(|m| {
                if x_s.contains(&m.0) {
                    return Some(m.0);
                }
                None
            })
            .collect();
        if m_s.is_empty() {
            continue;
        }
        let a_s: Vec<_> = lines
            .get(i + 2)
            .unwrap()
            .match_indices('A')
            .filter_map(|a| {
                if m_s.contains(&a.0) {
                    return Some(a.0);
                }
                None
            })
            .collect();
        if a_s.is_empty() {
            continue;
        }
        let s_s: Vec<_> = lines
            .get(i + 3)
            .unwrap()
            .match_indices('S')
            .filter_map(|s| {
                if a_s.contains(&s.0) {
                    return Some(s.0);
                }
                None
            })
            .collect();
        instances += s_s.len() as i32;
    }
    instances
}

fn find_vertical_backwards(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    let mut instances = 0;
    for (i, line) in lines.iter().enumerate() {
        if i + 4 > lines.len() {
            break;
        }
        let x_s: Vec<_> = line.match_indices('S').map(|x| x.0).collect();
        if x_s.is_empty() {
            continue;
        }
        let m_s: Vec<_> = lines
            .get(i + 1)
            .unwrap()
            .match_indices('A')
            .filter_map(|m| {
                if x_s.contains(&m.0) {
                    return Some(m.0);
                }
                None
            })
            .collect();
        if m_s.is_empty() {
            continue;
        }
        let a_s: Vec<_> = lines
            .get(i + 2)
            .unwrap()
            .match_indices('M')
            .filter_map(|a| {
                if m_s.contains(&a.0) {
                    return Some(a.0);
                }
                None
            })
            .collect();
        if a_s.is_empty() {
            continue;
        }
        let s_s: Vec<_> = lines
            .get(i + 3)
            .unwrap()
            .match_indices('X')
            .filter_map(|s| {
                if a_s.contains(&s.0) {
                    return Some(s.0);
                }
                None
            })
            .collect();
        instances += s_s.len() as i32;
    }
    instances
}

fn find_diagonal_right(input: &str) -> i32 {
    let line_length = input.lines().next().unwrap().len();
    let lines: Vec<_> = input.lines().collect();
    let mut instances = 0;
    for (i, line) in lines.iter().enumerate() {
        if i + 4 > lines.len() {
            break;
        }
        let x_s: Vec<_> = line
            .match_indices('X')
            .map(|x| x.0)
            .filter(|x| x + 4 <= line_length)
            .collect();
        if x_s.is_empty() {
            continue;
        }
        let m_s: Vec<_> = lines
            .get(i + 1)
            .unwrap()
            .match_indices('M')
            .filter_map(|m| {
                if m.0 == 0 {
                    return None;
                }
                if x_s.contains(&(m.0 - 1)) {
                    return Some(m.0);
                }
                None
            })
            .collect();
        if m_s.is_empty() {
            continue;
        }
        let a_s: Vec<_> = lines
            .get(i + 2)
            .unwrap()
            .match_indices('A')
            .filter_map(|a| {
                if a.0 == 0 {
                    return None;
                }
                if m_s.contains(&(a.0 - 1)) {
                    return Some(a.0);
                }
                None
            })
            .collect();
        if a_s.is_empty() {
            continue;
        }
        let s_s: Vec<_> = lines
            .get(i + 3)
            .unwrap()
            .match_indices('S')
            .filter_map(|s| {
                if s.0 == 0 {
                    return None;
                }
                if a_s.contains(&(s.0 - 1)) {
                    return Some(s.0);
                }
                None
            })
            .collect();
        instances += s_s.len() as i32;
    }
    instances
}

fn find_diagonal_left(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    let mut instances = 0;
    for (i, line) in lines.iter().enumerate() {
        if i + 4 > lines.len() {
            break;
        }
        let x_s: Vec<_> = line
            .match_indices('X')
            .map(|x| x.0)
            .filter(|x| *x >= 3)
            .collect();
        if x_s.is_empty() {
            continue;
        }
        let m_s: Vec<_> = lines
            .get(i + 1)
            .unwrap()
            .match_indices('M')
            .filter_map(|m| {
                if x_s.contains(&(m.0 + 1)) {
                    return Some(m.0);
                }
                None
            })
            .collect();
        if m_s.is_empty() {
            continue;
        }
        let a_s: Vec<_> = lines
            .get(i + 2)
            .unwrap()
            .match_indices('A')
            .filter_map(|a| {
                if m_s.contains(&(a.0 + 1)) {
                    return Some(a.0);
                }
                None
            })
            .collect();
        if a_s.is_empty() {
            continue;
        }
        let s_s: Vec<_> = lines
            .get(i + 3)
            .unwrap()
            .match_indices('S')
            .filter_map(|s| {
                if a_s.contains(&(s.0 + 1)) {
                    return Some(s.0);
                }
                None
            })
            .collect();
        instances += s_s.len() as i32;
    }
    instances
}

fn find_diagonal_right_backward(input: &str) -> i32 {
    let line_length = input.lines().next().unwrap().len();
    let lines: Vec<_> = input.lines().collect();
    let mut instances = 0;
    for (i, line) in lines.iter().enumerate() {
        if i + 4 > lines.len() {
            break;
        }
        let x_s: Vec<_> = line
            .match_indices('S')
            .map(|x| x.0)
            .filter(|x| x + 4 <= line_length)
            .collect();
        if x_s.is_empty() {
            continue;
        }
        let m_s: Vec<_> = lines
            .get(i + 1)
            .unwrap()
            .match_indices('A')
            .filter_map(|m| {
                if m.0 == 0 {
                    return None;
                }
                if x_s.contains(&(m.0 - 1)) {
                    return Some(m.0);
                }
                None
            })
            .collect();
        if m_s.is_empty() {
            continue;
        }
        let a_s: Vec<_> = lines
            .get(i + 2)
            .unwrap()
            .match_indices('M')
            .filter_map(|a| {
                if a.0 == 0 {
                    return None;
                }
                if m_s.contains(&(a.0 - 1)) {
                    return Some(a.0);
                }
                None
            })
            .collect();
        if a_s.is_empty() {
            continue;
        }
        let s_s: Vec<_> = lines
            .get(i + 3)
            .unwrap()
            .match_indices('X')
            .filter_map(|s| {
                if s.0 == 0 {
                    return None;
                }
                if a_s.contains(&(s.0 - 1)) {
                    return Some(s.0);
                }
                None
            })
            .collect();
        instances += s_s.len() as i32;
    }
    instances
}

fn find_diagonal_left_backward(input: &str) -> i32 {
    let lines: Vec<_> = input.lines().collect();
    let mut instances = 0;
    for (i, line) in lines.iter().enumerate() {
        if i + 4 > lines.len() {
            break;
        }
        let x_s: Vec<_> = line
            .match_indices('S')
            .map(|x| x.0)
            .filter(|x| *x >= 3)
            .collect();
        if x_s.is_empty() {
            continue;
        }
        let m_s: Vec<_> = lines
            .get(i + 1)
            .unwrap()
            .match_indices('A')
            .filter_map(|m| {
                if x_s.contains(&(m.0 + 1)) {
                    return Some(m.0);
                }
                None
            })
            .collect();
        if m_s.is_empty() {
            continue;
        }
        let a_s: Vec<_> = lines
            .get(i + 2)
            .unwrap()
            .match_indices('M')
            .filter_map(|a| {
                if m_s.contains(&(a.0 + 1)) {
                    return Some(a.0);
                }
                None
            })
            .collect();
        if a_s.is_empty() {
            continue;
        }
        let s_s: Vec<_> = lines
            .get(i + 3)
            .unwrap()
            .match_indices('X')
            .filter_map(|s| {
                if a_s.contains(&(s.0 + 1)) {
                    return Some(s.0);
                }
                None
            })
            .collect();
        instances += s_s.len() as i32;
    }
    instances
}
