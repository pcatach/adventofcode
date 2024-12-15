/*
Part 1: 3 tokens to push A, 1 token to push B
Given a list that describes the button configuration and prize location of different claw machines.

Approach: optimization problem defined on the space of "button pushes".
Let alpha be the number of button A pushes, and beta the number of button B pushes.
The optimization domain is the square between (alpha, beta) = (0,0) and (alpha, beta) = (100,100),
together with the intersection between the lines 
a_x * alpha + b_x * beta = X
a_y * alpha + b_y * beta = Y

As the solution is a point, the cost minimization doesn't impose a further constraint.
You just solve the equation and check if it's inside the square

alpha = (X * b_y  - Y * b_x) / (b_y * a_x - b_x * a_y)
beta = (Y - a_y * alpha) / b_y

Part 2: X,Y much larger, drop the (100, 100) constraint
*/
use std::io;
use std::time::Instant;

use regex::{Captures, Regex};

use utils::read_from_args;

type Position = i64;

fn main() -> io::Result<()> {
    let now = Instant::now();

    let input = read_from_args()?;
    let button_re = Regex::new(r"Button [A|B]: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    
    fn button_capture(capture: Captures) -> Option<[Position; 2]> {
        let (_, [x, y]) = capture.extract();
        Some([x.parse().unwrap(), y.parse().unwrap()])
    }

    fn prize_capture(capture: Captures) -> Option<[Position; 2]> {
        let (_, [x, y]) = capture.extract();
        Some([x.parse::<Position>().unwrap() + 10000000000000 as Position, y.parse::<Position>().unwrap() + 10000000000000 as Position])
    }
    
    let mut total_tokens: Position = 0;
    for description in input.split("\n\n") {
        let mut button_matches = button_re.captures_iter(&description);
        let [a_x, a_y]: [Position; 2] = button_matches.next().and_then(button_capture).unwrap();
        let [b_x, b_y]: [Position; 2] = button_matches.next().and_then(button_capture).unwrap();
        let [x, y]: [Position; 2] = prize_re.captures(&description).and_then(prize_capture).unwrap();

        if (x * b_y - y * b_x) % (b_y * a_x - b_x * a_y) != 0 {
            // no integer solution
            continue;
        }
        let alpha = (x * b_y - y * b_x) / (b_y * a_x - b_x * a_y);

        if (y - a_y * alpha) % b_y != 0 {
            // no integer solution
            continue;
        }
        let beta = (y - a_y * alpha) / b_y;

        // dbg!(alpha, beta);
        if alpha >= 0 // && alpha <= 100
        && beta >= 0 //  && beta <= 100 
        {
            let cost = 3 * alpha + beta;
            // dbg!(cost);
            total_tokens += cost;
        }
    }
    dbg!(total_tokens);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    
    Ok(())
}