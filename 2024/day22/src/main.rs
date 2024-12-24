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
use std::{collections::{HashMap, HashSet}, io, time::Instant};

use utils::read_from_args;

const MAX_NUMBERS: usize = 2000;

fn main() -> io::Result<()> {
    let time = Instant::now();
    let input = read_from_args()?;

    let mut sum: i64 = 0;
    let mut patterns_cache: HashMap<(i64, i64, i64, i64), i64> = HashMap::new(); // pattern of 4 diffs => total number of bananas
    let mut seen: HashSet<(i64, i64, i64, i64)> = HashSet::new();

    for line in input.lines() {
        let mut secret = line.parse::<i64>().unwrap();
        let mut prices = [0; MAX_NUMBERS];

        for price in prices.iter_mut() {
            secret = rng(secret);
            *price = secret % 10;
        }

        for i in 4..MAX_NUMBERS {
            let diffs = 
                (prices[i-3] - prices[i-4], prices[i-2] - prices[i-3], prices[i-1] - prices[i-2], prices[i] - prices[i-1]);
            if seen.insert(diffs) {
                *patterns_cache.entry(diffs).or_default() += prices[i];
            }
        }
        seen.clear();
        sum += secret;
    }
    dbg!(&patterns_cache.into_values().max());
    // let max_pattern = &patterns_cache.iter().max_by_key(|x| *x.1).unwrap();
    // dbg!(max_pattern);
    dbg!(sum);
    dbg!(time.elapsed());
    Ok(())
}

fn rng(s: i64) -> i64 {
    let mut n = ( (s << 6) ^ s) & 0xFFFFFF;
    n = (n >> 5) ^ n; // pruning not needed
    ((n << 11) ^ n) & 0xFFFFFF
}
