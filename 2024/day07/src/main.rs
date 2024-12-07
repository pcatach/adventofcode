/*
Part 1: you're given calibration equations with missing operators:
```
2: 1 1
4: 2 2
3: 2 1 1
5: 1 2
```
Your job is to find which equations are valid with the insertion of
"+" or "*" in the appropriate place, and return the sum of the total
calibration result for each valid equation.
```
2: 1+1
4: 2*2 OR 2+2
3: 2*1+1 OR 2+1*1
~5: 1?2~
Total valid = 2+4+3 = 9
```
Note: operators are always evaluated from left to right, not according
to precedence.

Approach: DFS with early stopping. Suppose we build a tree where
the root is the first number in the equation, and each children is
the result of adding or multiplying by the next number in the equation.
If the result is greater than the total result, we can ignore that branch.
If we reach a leaf (last number) and the result is equal to the total,
the equation is valid. If we looked at all the branches, the equation is
invalid.

Part 2: another operation 12 || 34 = 1234
*/
use std::io;

use utils::read_from_args;

fn main() -> io::Result<()> {
    let equations = read_from_args()?;

    let mut total_sum = 0;
    for line in equations.lines() {
        let (total_str, numbers_str) = line.split_once(": ").unwrap();
        let total: u128 = total_str.parse().unwrap();
        let numbers: Vec<u128> = numbers_str.split(" ").map(|n| n.parse().unwrap()).collect();

        if is_valid_equation(total, numbers) {
            // println!("{line} is valid");
            total_sum += total;
        } else {
            // println!("{line} is invalid");
        }
    };
    dbg!(total_sum);
    Ok(())
}

fn is_valid_equation(total: u128, numbers: Vec<u128>) -> bool {
    let mut stack: Vec<(u128, usize)> = Vec::new();
    // format of each tuple is (number, index_of_next_number)
    stack.push((numbers[0], 1));

    while stack.len() > 0 {
        let (current, index_next) = stack.pop().unwrap();
        if current > total {
            // no other operations in this branch can get to the total
            // about 2x speed improvement
            continue;
        } 
        
        if index_next == numbers.len() {
            // exhausted all numbers in list
            if current == total {
                return true;
            }
            continue;
        }
        let next = numbers[index_next];
        
        let sum = current + next;
        let mul = current * next;
        // let concat: u128 = (current.to_string() + &next.to_string()).parse().unwrap();
        // about 2x improvement
        let concat = 10_u128.pow(next.ilog10()+1) * current + next;

        stack.push((sum, index_next+1));
        stack.push((mul, index_next+1));
        stack.push((concat, index_next+1));
    };
    false
}