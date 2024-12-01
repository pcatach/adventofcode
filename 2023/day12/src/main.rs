// "borrowed" from https://github.com/sopyb/AoC/blob/main/2023/day_12/src/part1.rs
use itertools::Itertools;
use memoize::memoize;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let records = fs::read_to_string(file_path).expect("Could not read file");
    let sum_of_arrangements: usize = records
        .lines()
        .map(|line| {
            let (springs, counts) = line.split_once(' ').unwrap();

            // for part 2:
            let springs = (0..5).map(|_| springs).join("?");
            let counts = (0..5).map(|_| counts).join(",");

            let springs = springs.chars().collect();
            let counts = counts
                .split(',')
                .map(|number| number.parse().unwrap())
                .collect();

            compute_arrangements(springs, counts)
        })
        .sum();
    println!("{:?}", sum_of_arrangements)
}

#[memoize]
fn compute_arrangements(springs: Vec<char>, count: Vec<usize>) -> usize {
    // println!("{} {:?}", springs.iter().collect::<String>(), count);
    if springs.is_empty() {
        if count.is_empty() {
            // println!("yea boii");
            return 1;
        }
        // println!("nah");
        return 0;
    }

    match springs[0] {
        '.' => compute_arrangements(springs[1..].to_vec(), count),
        '?' => {
            let mut spring_alt = springs.clone();
            spring_alt[0] = '#';
            compute_arrangements(springs[1..].to_vec(), count.clone())
                + compute_arrangements(spring_alt, count)
        }
        '#' => {
            if count.is_empty() {
                return 0;
            }
            let expected_len = count[0];
            if springs.len() < expected_len {
                // there's no possibility of a match of size expected_length
                // println!("nah");
                return 0;
            }
            if !springs[..expected_len].contains(&'.') {
                return match springs.get(expected_len) {
                    Some('.') | None => {
                        // there's a dot after the match, so that's possible
                        compute_arrangements(springs[expected_len..].to_vec(), count[1..].to_vec())
                    }
                    Some('?') => {
                        // there's a ? after the match, so it's possible provided the ? is a dot
                        compute_arrangements(
                            springs[(expected_len + 1)..].to_vec(),
                            count[1..].to_vec(),
                        )
                    }
                    Some('#') => {
                        // no possibility of match
                        0
                    }
                    _ => panic!("WHAT?!"),
                };
            }
            // println!("nah");
            0
        }
        _ => panic!("WHAT!?"),
    }
}
