/*
Part 1: you are given a door code that must be typed into a numeric keypad arranged like
```
789
456
123
 0A
```
and a keypad like 
```
 ^A
<v>
```
the keypad controls a robot arm (robot A) that types into another keypad
that contorls another robot arm (robot B) that types into another keypad
that controls another robot arm (robot C) that types into the numeric keypad.
Robots A, B, and C start pointing at "A" on their respective keypads.

Robot is never allowed to pass through empty " " positions.

What's the shortest possible sequence of buttons you could press to cause robot
A to type a code on the numeric keypad?

The complexity of a code is given by the length of the shortest sequence, times
the numeric part of the code. Give the sum of the complexities of all the codes you're given.

Approach: we can solve this with multiple steps of pathfinding (say, using BFS). On the first step,
we find the shortest path on the numerical keypad (robot C). On the second step, we take as an input that 
previous shortest path and find the shortest path on the directions keypad that produces that path (robot B).
We repeat for robot A, and finally in the 4th and last BFS we do it for the outermost keypad.

That will not work. You can't find the shortest path one robot at a time, because there are multiple shortest paths
e.g. ^^> is equivalent to ^>^ but ^>^ takes longer for the robot above.
Maybe I can give extra points for same-direction sequences?

Another issue is that e.g. <^A vs ^<A should be weighted differently because < is farther from A than ^
the priority is N, E, D, W.

This approach doesn't work. I need instead to take all the robots into account at once when doing my path search.
So before saying that a path in the numeric keypad is the shortest, I must do a BFS on the directions keypad,
and recurse until I get to robot C
*/

use std::{collections::{HashMap, VecDeque}, io, usize};

use utils::{add_checked_direction, pause, read_from_args, Direction, DIRECTIONS4, E, N, S, W};

const NUMERIC_KEYPAD: &[&[char]] = &[
    &['7', '8', '9'],
    &['4', '5', '6'],
    &['1', '2', '3'],
    &[' ', '0', 'A']
];

const DIRECTIONS_KEYPAD: &[&[char]] = &[
    &[' ', '^', 'A'],
    &['<', 'v', '>']
];

const MAX_DEPTH: usize = 25;

fn main() -> io::Result<()> {
    let codes: Vec<Vec<char>> = read_from_args()?.lines().map(|l| l.chars().collect()).collect();
    let mut cache: HashMap<(usize, char, char), usize> = HashMap::new(); // depth, from, to => length

    let mut sum_complexities = 0;
    for code in codes {
        let mut length = 0;
        for i in 0..code.len() {
            let start = if i == 0 {'A'} else {code[i-1]};
            let end = code[i];
            length += solution(&mut cache, start, end, MAX_DEPTH);
        }
        println!("{}: {}", code.iter().collect::<String>(), length);
        sum_complexities += code[..code.len()-1].iter().collect::<String>().parse::<usize>().unwrap() * length;
    }
    dbg!(sum_complexities);
    Ok(())
}

fn solution(cache: &mut HashMap<(usize, char, char), usize>, from: char, to: char, depth: usize) -> usize {
    if let Some(&length) = cache.get(&(depth, from, to)) {
        return length;
    }

    let shortest_paths = bfs(from, to, if depth==MAX_DEPTH {NUMERIC_KEYPAD} else {DIRECTIONS_KEYPAD});
    if depth == 0 {
        return shortest_paths[0].len();
    }

    let mut min_length = usize::MAX;
    for path in shortest_paths {
        let mut length = 0;
        for i in 0..path.len() {
            let start = if i == 0 {'A'} else {path[i-1]};
            let end = path[i];
            length += solution(cache, start, end, depth-1);
        }

        if length < min_length {
            min_length = length;
        }
    }
    cache.insert((depth, from, to), min_length);
    min_length
}

fn bfs(
    from: char,
    to: char,
    keypad: &[&[char]]
) -> Vec<Vec<char>> {
    let mut shortest_paths: Vec<Vec<char>> = Vec::new();
    let mut shortest_path_len= usize::MAX;

    let mut queue: VecDeque<(char, Vec<char>)> = VecDeque::new();
    queue.push_back((from, Vec::new()));

    while let Some((key, mut path)) = queue.pop_front() {
        // longer than current shortest is forbidden
        if path.len() > shortest_path_len {
            // because it's a BFS, any remaining paths in the queue will be longer
            break;
        }
            
        if key == to {
            if path.len() <= shortest_path_len {
                path.push('A');
                shortest_path_len = path.len();
                shortest_paths.push(path);
            }
            continue;
        }

        for direction in DIRECTIONS4 {
            let Some(new_key) = get_new_key(key, &keypad, &direction) else {
                continue;
            };

            // gap is forbidden
            if new_key == ' ' {
                continue;
            }

            let mut new_path: Vec<char> = path.clone();
            new_path.push(map_to_char(&direction));
            queue.push_back((new_key, new_path));    
        }
    }
    shortest_paths
}

fn get_new_key(key: char, keypad: &[&[char]], direction: &Direction) -> Option<char> {
    let mut position = (0, 0);
    'outer: for i in 0..keypad.len() {
        for j in 0..keypad[0].len() {
            if keypad[i][j] == key {
                position = (i, j);
                break 'outer;
            }
        }
    }

    let Some(new_position) = add_checked_direction(position, *direction) else {
        return None;
    };
    if new_position.0 >= keypad.len() || new_position.1 >= keypad[0].len() {
        return None;
    }
    Some(keypad[new_position.0][new_position.1])
}

fn map_to_char(d: &Direction) -> char {
    match *d {
        N => '^',
        S => 'v',
        E => '>',
        W => '<',
        _ => unreachable!()
    }
}