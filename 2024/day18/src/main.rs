/*
Part 1: you are given a list of coordinates on a grid from 0 to 70 (or 0 to 6 in the example)
You start at (0, 0) and want to reach the exit at (70, 70) or (6, 6) in the example.
The coordinates in the list are blocked. What is the shortest path available after the first
1024 coordinates are blocked (12 in the example)?

Approach: BFS keeping track of the position and path.

Pop position + path from the queue. 
If position has been visited, terminate path.
If position is a wall or out of the map, terminate path.
If the path is longer than the shortest known path, terminate path.
If position is end, terminate the path and update shortest known path.
Then, for each of the available directions, push new path to the queue.

Part 2: from the list of coordinates, which is the first one that would
prevent the exit being reachable?

Approach: collect every possible path between start and end (so do the same BFS
but don't terminate unless position is a wall/out of map) - add a condition to terminate
if the path is a loop.

Find the first coordinate that blocks a bottleneck: a position in the intersection of all paths.

This is too slow... is there a way to collect the bottlenecks without having to generate every possible path?
You can take the shortest path and look for bottlenecks by checking positions where there's a wall (or out of map)
in two distinct directions.

*/
use std::{collections::{HashSet, VecDeque}, io, usize};

use utils::{add_checked_direction, pause, read_from_args, DIRECTIONS4};

const T: usize = 12;
// const T: usize = 1024;
const SIZE: usize = 6+1;
// const SIZE: usize = 70+1;

fn main() -> io::Result<()> {
    let input = read_from_args()?;
    let all_coordinates: Vec<(usize, usize)> = input.lines()
        .map(|line| {
            let coords = line.split_once(",").unwrap();
            (coords.0.parse::<usize>().unwrap(), coords.1.parse::<usize>().unwrap())
        }).collect();
    let coordinates: Vec<(usize, usize)> = all_coordinates.iter().take(T).cloned().collect();

    let start = (0, 0);
    let end = (SIZE-1, SIZE-1);

    let shortest_path = find_shortest_path(&coordinates, start, end);
    dbg!(shortest_path.len() - 1);

    let bottlenecks = find_bottlenecks(&coordinates, start, end);
    // dbg!(bottlenecks);
    // print_path(&coordinates, &Vec::from_iter(bottlenecks.iter().cloned()));
    for &coord in all_coordinates.iter().skip(T) {
        if bottlenecks.contains(&coord) {
            dbg!(coord);
        }
    }
    Ok(())
}

fn find_shortest_path(coordinates: &Vec<(usize, usize)>, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    let mut queue: VecDeque<((usize, usize), Vec<(usize, usize)>)> = VecDeque::new(); // (position, path)
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back((start, vec![start]));

    let mut shortest_path_size = usize::MAX;
    let mut shortest_path: Vec<(usize, usize)> = Vec::new();

    while !queue.is_empty() {
        let (position, path) = queue.pop_front().unwrap();

        if visited.contains(&position) {
            continue;
        }
        visited.insert(position);
        
        if coordinates.contains(&position)
          || position.0 >= SIZE
          || position.1 >= SIZE 
          || path.len() >= shortest_path_size {
            continue;
        }
        
        // print_path(&coordinates, &path);

        if position == end {
            if path.len() < shortest_path_size { 
                shortest_path_size = path.len();
                shortest_path = path;
            }
            continue;
        }

        for direction in DIRECTIONS4 {
            let Some(new_position) = add_checked_direction(position, direction) else {
                continue;
            };
            let mut new_path = path.clone();
            new_path.push(new_position);
            queue.push_back((new_position, new_path));
        }
        // dbg!(&queue);
    }
    shortest_path
}

fn find_bottlenecks(coordinates: &Vec<(usize, usize)>, start: (usize, usize), end: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut all_paths: Vec<HashSet<(usize, usize)>> = Vec::new();

    let mut queue: VecDeque<((usize, usize), Vec<(usize, usize)>)> = VecDeque::new(); // (position, path)
    queue.push_back((start, vec![start]));

    while !queue.is_empty() {
        let (position, path) = queue.pop_front().unwrap();
       
        if coordinates.contains(&position)
          || position.0 >= SIZE
          || position.1 >= SIZE {
            continue;
        }
        
        // print_path(&coordinates, &path);

        if position == end {
            all_paths.push(HashSet::from_iter(path.iter().cloned()));
            continue;
        }

        for direction in DIRECTIONS4 {
            let Some(new_position) = add_checked_direction(position, direction) else {
                continue;
            };
            if path.contains(&new_position) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(new_position);
            queue.push_back((new_position, new_path));
        }
        // dbg!(&queue);
    }
    // dbg!(&all_paths);
    all_paths.iter()
        .fold( all_paths[0].clone(), |acc, path| 
            acc.intersection(path).cloned().collect::<HashSet<(usize, usize)>>()
        )
}

#[allow(dead_code)]
fn print_path(coordinates: &Vec<(usize, usize)>, path: &Vec<(usize, usize)>) {
    for j in 0..SIZE {
        for i in 0..SIZE {
            if coordinates.contains(&(i, j)) {
                print!("#");
            } else if path.contains(&(i, j)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
    pause();
}