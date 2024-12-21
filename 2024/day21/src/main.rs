/*
Part 1: you are given a door code that must be typed into a numeric keypad arranged like
```
7 8 9
4 5 6
1 2 3
  0 A
```
and a keypad like 
```
  ^ A
< v >
```
the keypad controls a robot arm (robot A) that types into another keypad
that contorls another robot arm (robot B) that types into another keypad
that controls another robot arm (robot C) that types into the numeric keypad.
Robots A, B, and C start pointing at "A" on their respective keypads.

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
*/

use std::{collections::{HashSet, VecDeque}, io};

use utils::{add_checked_direction, read_from_args, Direction, DIRECTIONS4, E, N, S, W};

const NUMERIC_KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A']
];

const DIRECTIONS_KEYPAD: [[char; 3]; 2] = [
    [' ', '^', 'A'],
    ['<', 'v', '>']
];

fn main() -> io::Result<()> {
    let codes: Vec<Vec<char>> = read_from_args()?.lines().map(|l| l.chars().collect()).collect();
    
    let numeric_keypad: Vec<Vec<char>> = NUMERIC_KEYPAD.to_vec().iter().map(|l| l.to_vec()).collect();
    let directions_keypad: Vec<Vec<char>> = DIRECTIONS_KEYPAD.to_vec().iter().map(|l| l.to_vec()).collect();

    let mut sum_complexities = 0;
    for code in codes {
        let mut path = bfs(&numeric_keypad, (3, 2), &code);
        println!("{}: {} ({})", code.iter().collect::<String>(), path.iter().collect::<String>(), path.len());
        path = bfs(&directions_keypad, (0, 2),  &path);
        println!("{}: {} ({})", code.iter().collect::<String>(), path.iter().collect::<String>(), path.len());
        path = bfs(&directions_keypad, (0, 2),  &path);
        println!("{}: {} ({})", code.iter().collect::<String>(), path.iter().collect::<String>(), path.len());

        sum_complexities += code[..code.len()-1].iter().collect::<String>().parse::<usize>().unwrap() * path.len();
    }
    dbg!(sum_complexities);
    Ok(())
}

fn bfs(keypad: &Vec<Vec<char>>, start: (usize, usize), code: &Vec<char>) -> Vec<char> {
    let mut keys = code.iter();

    // a sequence of BFS for finding each of the keys (emptying the queue in turns)
    let mut shortest_path: Vec<char> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<((usize, usize), Vec<Direction>)> = VecDeque::new();
    queue.push_back((start, Vec::new()));
    
    let mut key = keys.next().unwrap();

    while !queue.is_empty() {
        let (position, path) = queue.pop_front().unwrap();
        // println!("{}, {}, path={}", position.0, position.1, path.iter().collect::<String>());

        // avoid visited nodes
        if visited.contains(&position) {
            continue;
        }
        visited.insert(position);

        if keypad[position.0][position.1] == *key {
            // found key
            shortest_path.extend(
                &path.iter().map(|d| map_to_char(d)).collect::<Vec<char>>()
            );
            shortest_path.push('A');

            // reset BFS
            visited = HashSet::new();
            queue = VecDeque::new();
            queue.push_back((position, Vec::new()));

            // start looking for next key
            match keys.next() {
                Some(k) => {key = k},
                None => break
            }
            continue;
        }

        for direction in DIRECTIONS4 {
            let Some(new_position) = add_checked_direction(position, direction) else {
                continue;
            };
            if new_position.0 >= keypad.len() || new_position.1 >= keypad[0].len() {
                continue;
            }
            // gap is forbidden
            if keypad[new_position.0][new_position.1] == ' ' {
                continue;
            }


            let mut new_path = path.clone();
            new_path.push(direction);
            queue.push_back((new_position, new_path));
        }
    }
    shortest_path
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