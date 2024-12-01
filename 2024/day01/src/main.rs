use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = read_from_args()?;
    // println!("{input}");

    let vecs = parse_into_vecs(&input);
    // println!("{:?}", vecs);

    let sum_of_diffs: u128 = sum_diffs(&mut vecs.clone());
    println!("{sum_of_diffs}");

    let similarity_score: u128 = compute_similarity(vecs);
    println!("{similarity_score}");

    Ok(())
}

fn compute_similarity(vectors: (Vec<u128>, Vec<u128>)) -> u128 {
    let mut map = HashMap::new();

    for value in vectors.1 {
        match map.get(&value) {
            Some(count) => {map.insert(value, count + 1);}
            None => {map.insert(value, 1);}
        }
    };
    // println!("{:?}", map);

    let mut similarity = 0;
    for value in vectors.0 {
        let count = map.get(&value);

        match count {
            Some(multiplier) => {similarity += value * multiplier}
            None => {}
        }
    };
    similarity
}

fn sum_diffs(vectors: &mut (Vec<u128>, Vec<u128>)) -> u128{
    sort(&mut vectors.0);
    sort(&mut vectors.1);

    vectors.0.iter().zip(&vectors.1).map(
        |(first, second)| first.abs_diff(*second)
    ).sum()
}

fn sort(vector: &mut Vec<u128>) {
    // Extra: implement from scratch with O(n log n)
    vector.sort()
}

fn read_from_args() -> io::Result<String> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    fs::read_to_string(file_path)
}

fn parse_into_vecs(input: &String) -> (Vec<u128>, Vec<u128>) {
    input.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let first = parts.next().unwrap().parse::<u128>().unwrap();
        let second = parts.next().unwrap().parse::<u128>().unwrap();
        (first, second)
    }).collect()
}