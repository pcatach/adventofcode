/*
Part 1: you are given a list of initial secret numbers for banana sellers:
```
1
10
100
2024
```
Each initial secret number evolves into the next secret number according to
the rule:
n = [(S << 8) ^ S] % 2**24
U = [(S >> 7) ^ S] % 2**24
V = [(S << 11) ^ S] % 2**24

What's the sum of the 2000th secret numbers for each initial number?

Part 2: the price (=#bananas) is given by the ones digit of each secret number.
The monkey will execute your trade when it first sees a sequence of 4 changes in price,
and then go to the next seler.
If it doesn'n see that sequence for a given banana seller, it will jump to the next one.

What sequence must the monkey look for in order to maximise the number of bananas purchased?
*/
use std::{collections::{HashMap, VecDeque}, io};

use utils::read_from_args;

const MAX_NUMBERS: usize = 2000;

fn main() -> io::Result<()> {
    let input = read_from_args()?;

    let mut sum: u64 = 0;
    let mut patterns_cache: HashMap<VecDeque<isize>, u64> = HashMap::new(); // pattern of 4 diffs => total number of bananas

    for line in input.lines() {
        let seed = line.parse::<u64>().unwrap();
        let mut current = seed;
        let mut price_diffs = VecDeque::new();

        let mut monkey_patterns: HashMap<VecDeque<isize>, u64> = HashMap::new();
        for _ in 0..MAX_NUMBERS {
            let next = rng(current);
            let price_diff = (next % 10) as isize - (current % 10) as isize;
            current = next;
            println!("{}, {price_diff}", current % 10);
            
            price_diffs.push_back(price_diff);
            if price_diffs.len() < 4 {
                continue;
            }
            monkey_patterns.entry(price_diffs.clone()).or_insert(current % 10);
            price_diffs.pop_front();
        }
        sum += current;

        // merge patterns
        for (k, v) in monkey_patterns.iter() {
            if let Some(count) = patterns_cache.get_mut(&k) {
                *count += v;
            } else {
                patterns_cache.insert(k.clone(), *v);
            }
        }
    }
    let max_pattern = &patterns_cache.iter().max_by_key(|x| *x.1).unwrap();
    dbg!(max_pattern);
    dbg!(sum);
    Ok(())
}

fn rng(s: u64) -> u64 {
    let mut n = ( (s << 6) ^ s) & (2_u64.pow(24) - 1);
    n = ( (n >> 5) ^ n) & (2_u64.pow(24) - 1);
    n = ( (n << 11) ^ n) & (2_u64.pow(24) - 1);
    return n
}
