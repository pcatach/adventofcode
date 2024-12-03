/* Part 1: Memory (input) is corrupted: invalid characters
should be ignored.
Only instructions like `mul(X,Y)` are valid.
Evaluate valid instructions and return the sum.

Part 2: Instruction `do()` enables all future `mul` instructions,
instruction `don't()` disables. Enabled in the beginning.

Build a list of conditional positions e.g.
[13, 25, 60]
  ^   ^   ^
dont  do dont

if you find a mul() match at potision i, you can
activate/deactivate it depending on whether find(cond, i)
is odd/even
*/

use std::io;

use regex::{Captures, Regex};

use utils::read_from_args;

fn main() -> io::Result<()> {
    let mul_re = Regex::new(r"mul\((?<X>\d{1,3}),(?<Y>\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"(do\(\)|don't\(\))").unwrap();

    let memory = read_from_args()?;
    // dbg!(&memory);

    let conditionals: Vec<(usize, bool)> = do_re.find_iter(&memory).map(
        |m| match m.as_str() {
            "do()" => (m.start(), true),
            "don't()" => (m.start(), false),
            _ => panic!("Unknown instruction.")
        }
    ).collect();
    // dbg!(&conditionals);

    let multiplications: u32 = mul_re.captures_iter(&memory).filter_map(
    |capture| {
        let position = capture.get(0)?.start();
        let enabled = check_mul_enabled(&conditionals, position);
        let x = parse_capture("X", &capture)?;
        let y = parse_capture("Y", &capture)?;
        if enabled {
            return Some(x*y);
        } else {
            return Some(0);
        }

    }).sum();
    dbg!(multiplications);

    Ok(())
}

fn check_mul_enabled(conditionals: &[(usize, bool)], position: usize) -> bool {
    // Extra: implement binary search from scratch
    let index = match conditionals.binary_search_by_key(&position, |&(pos, _)| pos) {
        Ok(i) => i,
        Err(i) => i
    };
    if index == 0 {return true;}
    return conditionals[index-1].1
}

fn parse_capture(name: &str, capture: &Captures) -> Option<u32> {
    capture.name(name)?.as_str().parse().ok()
}
