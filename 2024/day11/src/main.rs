/*
Part 1: you are given a line of numbered stones. At each time step, 
stones change according to the first of:
- If stone = 0, it is replaced by stone = 1
- If stone has an even number of digits, it is replaced by two stones,
one with the left half, the other with the right half (without leading zeroes)
- Else, stone is replaced by stone * 2024

0 1 10 99 999
1 2024 1 0 9 9 2021976

How many stones will you have after 25 time steps?

Part 2: 75? :lol:
Array and numbers get too large. Using a counter.
*/
use std::{collections::HashMap, io};

use utils::read_from_args;

type Stone = u64;

fn main() -> io::Result<()> {
    let stones_input = read_from_args()?;
    // let mut stones: Vec<Stone> = stones_input.split(" ").map(|s| s.parse().unwrap()).collect();
    let mut stones: HashMap<Stone, u64> = HashMap::new();
    stones_input.split(" ").for_each(
        |s| {
            stones.entry(
            s.parse().unwrap()
            ).and_modify(|count| *count += 1)
            .or_insert(1);
    });
    // dbg!(&stones);

    // let max_steps = 6;
    // let max_steps = 25; // 202019
    let max_steps = 75;

    let mut cache: HashMap<Stone, Vec<Stone>> = HashMap::new();

    for _step in 1..max_steps+1 {
        let mut next_stones: HashMap<Stone, u64> = HashMap::new();

        for (stone, count) in stones {
            let new_stones = cache.entry(stone).or_insert(apply_rules(stone));

            for new_stone in new_stones {
                next_stones.entry(*new_stone).and_modify(|c| *c += count).or_insert(count);
            }
        }
        stones = next_stones;

        // dbg!(_step, count_stones(&stones));
    }
    dbg!(count_stones(&stones));

    Ok(())
}

fn count_stones(stones: &HashMap<Stone, u64>) -> u64 {
    stones
    .iter()
    .fold(0, |sum, (_, count)| sum + *count)
}

fn apply_rules(stone: Stone) -> Vec<Stone> {
    if stone == 0 {
        return vec![1];
    }
    
    let num_digits = (stone as f64).log10().floor() as u32 + 1;
    if num_digits % 2 == 0 {
        let half = num_digits / 2;
        let factor = (10 as Stone).pow(half);

        let right_half = stone % factor;
        let left_half = stone / factor;

        return vec![left_half, right_half];
    }

    return vec![stone * 2024];
}