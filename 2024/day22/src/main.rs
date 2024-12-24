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
use std::{collections::{HashMap, HashSet}, io};

use utils::read_from_args;

const MAX_NUMBERS: usize = 2000;

fn main() -> io::Result<()> {
    let input = read_from_args()?;

    let mut sum: u64 = 0;
    let mut patterns_cache: HashMap<[isize; 4], u64> = HashMap::new(); // pattern of 4 diffs => total number of bananas
    let mut seen: HashSet<[isize; 4]> = HashSet::new();

    for line in input.lines() {
        let mut secret = line.parse::<u64>().unwrap();
        let mut previous_price = secret % 10;

        let mut price_diffs = [0; 4];

        for i in 0..MAX_NUMBERS {
            secret = rng(secret);
            let price = secret % 10;
            let price_diff = price as isize - previous_price as isize;
            
            price_diffs = [price_diffs[1], price_diffs[2], price_diffs[3], price_diff];
            if i <= 3 {
                continue;
            }
            
            if !seen.contains(&price_diffs) {
                seen.insert(price_diffs);
                *patterns_cache.entry(price_diffs).or_insert(0) += price;
            }

            previous_price = price;
        }
        seen.clear();
        sum += secret;
    }
    let max_pattern = &patterns_cache.iter().max_by_key(|x| *x.1).unwrap();
    dbg!(max_pattern);
    dbg!(sum);
    Ok(())
}

fn rng(s: u64) -> u64 {
    let mut n = ( (s << 6) ^ s) & 0xFFFFFF;
    n = (n >> 5) ^ n; // pruning not needed
    ((n << 11) ^ n) & 0xFFFFFF
}
