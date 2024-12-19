/*
Part 1 you are given a list of towel patterns such as
```
r, wr, b, g, bwu, rb, gb, br
```

And a list of designs such as
```
brwrr
bggr
ubwu
```

How many of these designs are possible with the available towel patterns?

brwrr is possible with br wr r
bggr is possible with bggr
ubwu is impossible

Approach: divide and conquer. For each pattern, check if design[..pattern.len()] == pattern.
If yes, recurse on design[pattern.len()], if not return false.
The base case is when design.len() == 0, which returns true.

If the design size is N and each pattern's size is K

T(N) = T(N-K) + O(1)
T(N) = O(N/K)

Can use caching to speed up.
I can use only patterns that have the correct length (< design.length())

I can also restrict the search space by an initial pass where I collect only those patterns 
that have the same colors as the design.

Another thing I can do is checking that the design length is divisible by the GCD of the pattern lengths.
Update: that doesn't work because the GCD of the pattern lengths is 1 :(

Part 2: for each design, compute the number of different possible pattern combinations
*/

use std::collections::HashSet;
use std::time::Instant;
use std::{collections::HashMap, io};

use gcd::binary_usize;

use utils::read_from_args;

fn main() -> io::Result<()> {
    let now = Instant::now();

    let input = read_from_args()?;
    let lines = input.split_once("\n\n").unwrap();

    let patterns: HashSet<&str> = lines.0.split(", ").collect();
    let designs: Vec<&str> = lines.1.lines().collect();

    let mut cache: HashMap<String, bool> = HashMap::new();

    let mut num_possible = 0;
    let mut total_count = 0;
    for design in designs {
        total_count += 1;
        let is_possible = is_possible(design, &patterns, &mut cache);
        // dbg!(design, is_possible);
        if !is_possible {
            continue;
        }
        num_possible += 1;
    }
    dbg!(total_count);
    dbg!(num_possible);    

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}

fn is_possible(design: &str, patterns: &HashSet<&str>, cache: &mut HashMap<String, bool>) -> bool {
    let cached = cache.get(design);
    match cached {
        Some(&value) => {
            return value
        },
        None => {}
    }

    if design.len() == 0 {
        return true;
    }

    // dbg!(&design);
    // let mut new_patterns = patterns.clone();
    for &pattern in patterns {
        if pattern.len() > design.len() {
            // new_patterns.remove(pattern);
            continue;
        }

        if &design[..pattern.len()] == pattern 
        && is_possible(&design[pattern.len()..], &patterns, cache) {
            cache.insert(design.to_string(), true);
            return true;
        }
    }
    cache.insert(design.to_string(), false);
    false
}


#[allow(dead_code)]
fn check_divisible(design: &str, patterns: &Vec<&str>) -> bool {
    let d = design.len();
    let p: Vec<usize> = patterns.iter().map(|p| p.len()).collect();

    let gcd: usize = p.iter().fold(p[0], |g, &p_i| binary_usize(g, p_i));
    dbg!(gcd);
    d % gcd == 0
}
