use std::{fs, usize};

fn main() {
    let file_path = "input.txt";
    let patterns_input = fs::read_to_string(file_path).expect("Could not find file");

    let patterns = parse_patterns(&patterns_input);
    let mut horizontal_lines: Vec<usize> = Vec::new();
    let mut vertical_lines: Vec<usize> = Vec::new();

    for pattern in patterns.iter() {
        println!("{}\n", pattern);
        let pattern_rows: Vec<String> = pattern.lines().map(|s| s.to_string()).collect();
        if let Some(n) = find_reflection(pattern_rows) {
            horizontal_lines.push(n);
            continue;
        }

        // no horizontal lines found, look for vertical
        let cols = pattern.lines().next().unwrap().chars().count();
        let pattern_cols: Vec<String> = (0..cols)
            .map(|i| {
                let r: String = pattern
                    .lines()
                    .map(|line| line.chars().nth(i).unwrap())
                    .collect();
                r
            })
            .collect();
        if let Some(n) = find_reflection(pattern_cols) {
            vertical_lines.push(n)
        }
    }
    println!("horizontal lines: {:?}", horizontal_lines);
    println!("vertical lines: {:?}", vertical_lines);
    println!(
        "Summary: {}",
        vertical_lines.iter().sum::<usize>() + 100 * horizontal_lines.iter().sum::<usize>()
    )
}

fn find_reflection(patterns: Vec<String>) -> Option<usize> {
    for i in 0..(patterns.len() - 1) {
        let bottom_range = (i + 1)..(2 * (i + 1)).min(patterns.len());
        let top_range = (1 + i).saturating_sub(bottom_range.len())..=i;
        if is_reflection(&patterns[top_range], &patterns[bottom_range]) == 1 {
            // tolerate 1 smudge of difference
            return Some(i + 1);
        }
    }
    None
}

// fn is_reflection(pattern1: &[String], pattern2: &[String]) -> bool {
fn is_reflection(pattern1: &[String], pattern2: &[String]) -> usize {
    // instead of returning a bool, we return the number of differences
    if pattern1.len() != pattern2.len() {
        return pattern1.len() * pattern1.iter().next().unwrap().len();
    }
    let mut number_of_differences = 0;
    for i in 0..pattern1.len() {
        let reflection = pattern1.len() - 1 - i;
        number_of_differences += pattern1[i]
            .chars()
            .zip(pattern2[reflection].chars())
            .filter(|x| x.0 != x.1)
            .count();
    }
    number_of_differences
}

fn parse_patterns(input: &str) -> Vec<&str> {
    return input.split("\n\n").collect();
}
