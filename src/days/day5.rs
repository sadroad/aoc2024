use crate::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day5 {}

impl Solution for Day5 {
    fn solution(input: &str) -> (Option<i64>, Option<i64>) {
        let mut rules = HashMap::new();
        let mut lines = input.lines();
        for order_rule in lines.by_ref() {
            if order_rule.is_empty() {
                break;
            }
            let (prior, after) = order_rule.split_once("|").unwrap();
            rules.entry(prior).or_insert_with(Vec::new).push(after);
        }
        let mut total_middle = 0;
        let mut total_incorrect_middle = 0;
        for update in lines {
            let mut initially_correct = true;
            let mut prior_pages = HashSet::new();
            let mut update_pages = vec![];
            for page in update.split(",") {
                let rule = rules.get(page);
                prior_pages.insert(page);
                update_pages.push(page);
                if let Some(edge) = rule {
                    for node in edge {
                        if initially_correct && prior_pages.contains(node) {
                            initially_correct = false;
                        }
                    }
                }
            }
            if initially_correct {
                total_middle += update_pages
                    .get(update_pages.len() / 2)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();
            } else {
                update_pages = topological_sort(&update_pages, &prior_pages, &rules);
                total_incorrect_middle += update_pages
                    .get(update_pages.len() / 2)
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();
            }
        }
        (Some(total_middle), Some(total_incorrect_middle))
    }
}
// much bad efficient
fn topological_sort<'a>(
    graph: &[&'a str],
    graph_set: &HashSet<&'a str>,
    edges: &HashMap<&'a str, Vec<&'a str>>,
) -> Vec<&'a str> {
    let mut sorted = Vec::new();
    let mut visited = HashSet::new();

    for node in graph {
        visit(node, edges, &mut visited, &mut sorted, &graph_set);
    }

    sorted.reverse();
    sorted
}
fn visit<'a>(
    node: &'a str,
    rules: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
    sorted: &mut Vec<&'a str>,
    graph_set: &HashSet<&'a str>,
) {
    if visited.contains(node) {
        return;
    }
    visited.insert(node);
    if let Some(edges) = rules.get(node) {
        for &edge in edges {
            if graph_set.contains(&edge) {
                visit(edge, &rules, visited, sorted, graph_set);
            }
        }
    }
    sorted.push(node);
}
