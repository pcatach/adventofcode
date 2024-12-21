/*
Part 1: You are given a "program racetrack" map with track ("."), walls ("#"),
a start ("S") and an end ("E") positions. There is a single path from S to E.
Programs move in each of the 4 directions every 1 picosecond, but they are allowed to
"cheat": once during the race, for 2 picoseconds, they can pass through walls.
Each cheat has a start position, where the cheat is activated and the program
can pass through a wall, and an end position, where the cheat is deactivated
and the program must be on track.

How many cheats will save at least 100 picoseconds?

The path can be found with a simple DFS. Each position in the path
gets assigned a number (of picoseconds).
For each position in the path with time < TOTAL - 100, we can activate the cheat and 
look for any directions that return to the path. Let's say the cheat cuts the path
between t=X and t=Y. That means that, with the cheat, the total time of the path
will get reduced from T to T - (Y - X) + 2

Part 2: Cheats are actually up to 20 ps long 

Instead of checking for the 4 directions, we can check for any positions in the path
that are within 20 movements of the cheat position.
That is, we need to find all track positions that are within a manhattan radius = 20
of the cheat start position. In addition, any cheat within a manhattan radius of R
can be achieved in _ possible ways.
*/
use std::io;

use utils::{add_direction, read_array_from_string, read_from_args, DIRECTIONS4};

const MIN_TIME_SAVING: usize = 100;
const CHEAT_SIZE: usize = 20;

fn main() -> io::Result<()> {
    let map = read_array_from_string(read_from_args().unwrap());

    let mut start = (0, 0);
    let mut end= (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            match map[i][j] {
                'S' => start = (i, j),
                'E' => end = (i, j),
                _ => continue
            }
        }
    }

    let path = find_path(&map, start, end);
    // print_path(&map, &path);

    let total_time = path.len() - 1;
    dbg!(total_time);

    // need to check every possible cheat position along the path, as long
    // as a > 100 ps saving is possible.
    // That is impossible once there are fewer than 100ms remining till the end.
    let max_cheat_time = path.len() - MIN_TIME_SAVING;
    // each entry represents the time saving of a cheat
    let mut cheat_savings: Vec<usize> = Vec::new();

    for (t1, &cheat_position) in path[..max_cheat_time].iter().enumerate() {
        // find directions where the cheat cuts the path

        // for direction in DIRECTIONS4 {
        //     let cheat_start = add_direction(cheat_position, direction);
        //     if map[cheat_start.0][cheat_start.1] != '#' {
        //         continue;
        //     }
        //     let cheat_end = add_direction(cheat_start, direction);
        //     let Some(t2) = path.iter().position(|&pos| pos == cheat_end) else {
        //         continue;
        //     };

        //     if t2 <= t1 {
        //         continue;
        //     }
        //     cheat_savings.push((t2 - t1) - 2);
        // }

        // find cheats within a 20 movement radius that cut the path
        cheat_savings.extend(
            generate_cheat_savings(
                t1, cheat_position, &path
            )
        );
    }
    dbg!(cheat_savings.iter().filter(|&val| *val >= MIN_TIME_SAVING).count());
    Ok(())
}

fn generate_cheat_savings(
    t1: usize, 
    cheat_position: (usize, usize),
    path: &Vec<(usize, usize)>, 
) -> Vec<usize> {
    let mut cheat_savings: Vec<usize> = Vec::new();
    for (t2, &(i, j)) in path.iter().enumerate() {
        let cheat_distance = i.abs_diff(cheat_position.0) + j.abs_diff(cheat_position.1);
        if cheat_distance > CHEAT_SIZE {
            continue;
        }
        if t2 <= t1 + cheat_distance {
            continue;
        }
        cheat_savings.push((t2 - t1) - cheat_distance)
        
    }
    cheat_savings
}

fn find_path(map: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    // DFS
    let mut stack: Vec<(usize, usize)> = Vec::new();
    stack.push(start);
    // making use of the fact that there's only one path
    // so can push every new available direction we find
    let mut path: Vec<(usize, usize)> = Vec::new();
    // the time is just given by the position's position in the path vector

    while !stack.is_empty() {
        let position = stack.pop().unwrap();

        if path.contains(&position) {
            continue;
        }
        path.push(position);
        
        if position == end {
            break;
        }

        for direction in DIRECTIONS4 {
            let new_position = add_direction(position, direction);

            if map[new_position.0][new_position.1] != '#' {
                stack.push(new_position);
            }
        }
    }
    path
}

#[allow(dead_code)]
fn print_path(map: &Vec<Vec<char>>, path: &Vec<(usize, usize)>) {
    for (i, row) in map.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if path.contains(&(i, j)) {
                print!("O");
            } else {
                print!("{tile}");
            } 
        }
        println!();
    }
}
