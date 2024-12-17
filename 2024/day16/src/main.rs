/*
Part 1: you're given a reindeeer maze with tiles marked S (start) and E (end). 
The reinteed maze starts at S facing east, and can move forward (1 point) or
turn 90 degrees (1000 points).

What's the lowest score possible?

Approach: it's a path finding problem which can be solved with a DFS or BFS.
Each edge is weighted depending on whether it's a rotation from the current
direction or not.
For a DFS you need to follow a trail all the way until S or a dead end until
you can explore a different trail, for a BFS you can explore many trails 
at the same time so I think a BFS might be better, but they both have the same
worst case complexity.

Termination conditions:
    - Reached E
    - Current score >= minimum score
    - If I revisit the same tile, I can terminate one of the paths and proceed only with the one with lowest score.

Part 2: how many tiles are part of at least one of the lowest score paths through the maze?

Approach: again BFS from S.
In order to know the full path, I have to also maintain the path so far in each element of the queue.
, with termination conditions:
    - Reached E
    - Current score > minimum score (note that if ==, should add the path to seats and keep going)
*/
use std::{collections::{HashMap, HashSet, VecDeque}, io, usize};

#[allow(unused_imports)]
use utils::{Direction, DIRECTIONS4, E, add_direction, pause, read_array_from_string, read_from_args};

fn main() -> io::Result<()> {
    let input = read_from_args()?;
    let maze = read_array_from_string(input);

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for (i, row) in maze.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            match tile {
                'S' => {start = (i, j);},
                'E' => {end = (i, j);},
                _ => continue
            }
        }
    }

    let (minimum_score, num_seats) = find_minimum_score(&maze, start, end, E);
    dbg!(minimum_score, num_seats);

    Ok(())
}

fn find_minimum_score(
    maze: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
    starting_direction: Direction
) -> (usize, usize) {
    let mut queue:
        VecDeque<((usize,usize), Direction, usize, HashSet<(usize, usize)>)>
         = VecDeque::new(); // ((i, j), direction, score, path)

    let mut path: HashSet<(usize, usize)> = HashSet::new();
    let mut seats: HashSet<(usize, usize)> = HashSet::new();
    let mut visited_tiles: HashMap<((usize, usize), Direction), usize> = HashMap::new();

    path.insert(start);
    queue.push_back((start, starting_direction, 0, path));

    let mut minimum_score = usize::MAX;

    while !queue.is_empty() {
        let (tile, current_direction, score, path) = queue.pop_front().unwrap();

        // if I've been here before, terminate the longest path
        match visited_tiles.get_mut(&(tile, current_direction)) {
            None => {
                visited_tiles.insert((tile, current_direction), score); 
            },
            Some(existing_score) => {
                if score <= *existing_score {
                    *existing_score = score;
                } else {
                    continue;
                }
            }
        }

        // print_path(&maze, &path);
        // pause();

        if tile == end {
            if score < minimum_score {
                minimum_score = score;
                seats = path;
            } else if score == minimum_score {
                seats.extend(path);
            }
            continue;
        }
        
        for direction in DIRECTIONS4 {
            let new_tile = add_direction(tile, direction);
            // skip walls
            if maze[new_tile.0][new_tile.1] == '#' {
                continue;
            }
            // never go back
            if direction == -current_direction {
                continue;
            }

            let new_score = score + if direction == current_direction {
                1
            } else {
                1001
            };

            let mut updated_path = path.clone();
            updated_path.insert(new_tile);
            queue.push_back((new_tile, direction, new_score, updated_path));
        }
    }
    (minimum_score, seats.len())
}

#[allow(dead_code)]
fn print_scores(maze: &Vec<Vec<char>>, visited_tiles: &HashMap<(usize, usize), usize>) {
    for (i, row) in maze.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile == '#' {
                print!("{tile:6}");
            } else {
                let score = visited_tiles.get(&(i, j)).unwrap_or(&0);
                print!("{score:6}")
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_path(maze: &Vec<Vec<char>>, path: &HashSet<(usize, usize)>) {
    for (i, row) in maze.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if matches!(*tile, '#'|'E'|'S') {
                print!("{tile}");
            } else if path.contains(&(i, j)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}