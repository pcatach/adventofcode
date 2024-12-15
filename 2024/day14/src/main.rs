/*
Part 1: Given a list of robot positions (tiles) and velocities (tiles per second).
Robots do not colide, and wrap around the map when they reach the edges.
The "safety factor" is given by the product of the number of robots in each quadrant.
Robots exactly in the middle don't count.
What will the safety factor be in 100 seconds?

Approach: each robot position will be given by

X = X0 + Vx*100 (mod H)
Y = Y0 + Vy*100 (mod W)

Part 2: how long until robots line up in a christmas tree?
Easiest way to do 
$ cargo run input.txt > output.txt
$ grep "############" output.txt --context=30
*/
use std::{collections::HashMap, io};

use utils::read_from_args;

fn main() -> io::Result<()> {
    let input = read_from_args()?;
    // inverting the coordinates because it's easier
    // width -> height
    // height -> width
    let height: i32 = 101;
    let width: i32 = 103;
    let time = 100;

    let mut initial_positions: Vec<[i32; 2]> = Vec::new();
    let mut velocities: Vec<[i32; 2]> = Vec::new();

    for line in input.lines() {
        let (position_part, velocity_part) = line.split_once(" ").unwrap();

        let position_raw = position_part.split_once("=").unwrap().1
            .split_once(",").unwrap();
        let velocity_raw = velocity_part.split_once("=").unwrap().1
            .split_once(",").unwrap();

        let position: [i32; 2] = [position_raw.0.parse().unwrap(), position_raw.1.parse().unwrap()];
        let velocity: [i32; 2] = [velocity_raw.0.parse().unwrap(), velocity_raw.1.parse().unwrap()];

        initial_positions.push(position);
        velocities.push(velocity);
    }

    let positions = update_robots(&initial_positions, &velocities, time, height, width);
    
    let counts = count_robots(&positions);
    let safety_factor = compute_safety_factor(counts, height, width);
    dbg!(safety_factor);

    // search for christmas tree...
    for t in 0..10000 {
        let positions = update_robots(&initial_positions, &velocities, t, height, width);
        println!("{t}");
        print_robots(&positions, height, width);
    }

    Ok(())
}

fn update_robots(positions: &Vec<[i32; 2]>, velocities: &Vec<[i32; 2]>, time: i32, height: i32, width: i32) -> Vec<[i32; 2]> {
    positions.iter().enumerate().map(|(i, position)| {
        let velocity = velocities[i];
        [
            (position[0] + velocity[0] * time).rem_euclid(height),
            (position[1] + velocity[1] * time).rem_euclid(width),
        ]
    }).collect()
}

fn compute_safety_factor(counts: HashMap<(i32, i32), usize>, height: i32, width: i32) -> usize {
    let quadrant_counts = counts.iter()
    .fold([0, 0, 0, 0], |q, entry| {
            let (&(i, j), &count) = entry;
            [
                q[0] + count * (i < height / 2 &&  j < width / 2) as usize,
                q[1] + count * (i > height / 2 &&  j < width / 2) as usize,
                q[2] + count * (i < height / 2 &&  j > width / 2) as usize,
                q[3] + count * (i > height / 2 &&  j > width / 2) as usize
            ]
        }
    );
    quadrant_counts.iter().fold(1_usize, |acc, &q| acc * q)
}

#[allow(dead_code)]
fn print_robots(positions: &Vec<[i32; 2]>, height: i32, width: i32) {
    let counts = count_robots(positions);

    println!();
    for j in 0..width as i32 {
        for i in 0..height as i32 {
            match counts.get(&(i, j)) {
                None => print!("."),
                Some(_) => print!("#")
            }
        }
        println!();
    }
}

fn count_robots(positions: &Vec<[i32; 2]>) -> HashMap<(i32, i32), usize> {
    let mut counts: HashMap<(i32, i32), usize> = HashMap::new();
    positions.iter().for_each(
        |position| {
            counts.entry((position[0], position[1]))
                .and_modify(|c| *c += 1).or_insert(1);
        }
    );
    counts
}
