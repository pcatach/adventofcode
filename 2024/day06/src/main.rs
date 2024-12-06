/*
Part 1: take a map like
..#..
#....
..^.#
..#..
Where # are obstacles and ^ is a guard facing up.
Guard walks forward, turns 90 degrees when hits an obstacle.
How many different positions in the map will the guard visit, including the starting position?

brute force: step by step walk, keeping track of visited positions
and following rules at each time.
Assuming no cycles.

Part 2: find new obstacle positions that would result in a cycle

Also just bruteforcing for each map variation... small optimisation possible
when adding an obstacle to the map would not affect the original guard's route. 

Let's say the map is of size N.
One step by step walk can be done in time 
*/
use std::{collections::HashSet, io};

use utils::{read_from_args, read_array_from_string};
 
fn main() -> io::Result<()> {
    let map_text = read_from_args()?;
    let map = read_array_from_string(map_text);
    let height = map.len();
    let width = map[0].len();
    // dbg!(&map);

    let guard = get_guard(&map, height, width);

    let guard_movements = walk(
        guard,
        &map,
        height,
        width
    ).unwrap();

    let guard_positions: Vec<(usize, usize)> = guard_movements.iter().map(
        |guard| (guard.0, guard.1)
    ).collect::<HashSet<(usize, usize)>>()
    .into_iter().collect();

    let sum_unique_positions = guard_positions.iter().count();
    dbg!(sum_unique_positions);

    let mut obstacle_positions = 0;
    for (i, j) in guard_positions.iter() {
        let mut variation = map.clone();
            
        // if it's current guard position
        if variation[*i][*j] != '.' {
            continue;
        }
        variation[*i][*j] = '#';

        match walk(guard, &variation, height, width) {
            Ok(_) => continue,
            Err(_) => {
                eprintln!("Cycle found for obstacle at ({i}, {j})");
                obstacle_positions += 1
            }
        }
    }
    dbg!(obstacle_positions);

    Ok(())
}

fn get_guard(map: &Vec<Vec<char>>, height: usize, width: usize) -> (usize, usize, char) {
    let mut guard: (usize, usize, char) = (0, 0, ' ');
    'outer: for i in 0..height {
        for j in 0..width {
            let value = map[i][j];
            if !['.', '#'].contains(&value) {
                guard = (i, j, value);
                break 'outer;
            }
        }
    }
    guard
}

fn walk(
    mut guard: (usize, usize, char),
    map: &Vec<Vec<char>>,
    height: usize,
    width: usize
) -> Result<Vec<(usize, usize, char)>, &'static str> {
    let mut guard_positions: Vec<(usize, usize, char)> = Vec::new();
    guard_positions.push(guard);
    loop {
        // dbg!(guard);

        let next_guard = match guard_step(guard, height, width) {
            Ok(next_guard) => next_guard,
            Err(_) => {break}
        };
        // dbg!(next_guard);

        // check for obstacle
        if map[next_guard.0][next_guard.1] == '#' {
            guard.2 = match guard.2 {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("Unknown direction")
            };
            continue;
        }

        // update guard position
        guard = next_guard;
        if guard_positions.contains(&guard) {
            return Err("Cycle found.")
        }
        guard_positions.push(guard);
    }
    Ok(guard_positions)
}

fn guard_step(
    guard: (usize, usize, char), 
    height: usize, 
    width: usize
) -> Result<(usize, usize, char), &'static str> {
    let (i, j, direction) = guard;
    let (next_i, next_j): (isize, isize) = match direction {
        '^' => (i as isize - 1, j as isize),
        'v' => (i as isize + 1, j as isize),
        '>' => (i as isize, j as isize + 1),
        '<' => (i as isize, j as isize - 1),
        _ => panic!("Unknown direction."),
    };

    if !check_guard_in_map(next_i, next_j, height, width) {
        return Err("Guard moved out of bounds.");
    }
    Ok((next_i as usize, next_j as usize, direction))
}

fn check_guard_in_map(i: isize, j: isize, height: usize, width: usize) -> bool {
    0 <= i
    && i < height as isize
    && 0 <= j
    && j < width as isize 
}