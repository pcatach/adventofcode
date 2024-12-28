/*
You are given key and lock schematics. 
Locks have the top row filled with #, keys have the bottom row filled with #
They can be represented as a sequence of pin heights:
0,5,3,4,3:"
#####
.####
.####
.####
.#.#.
.#...
.....
5,0,2,1,3:
.....
#....
#....
#...#
#.#.#
#.###
#####

How many unique lock/key pairs fit together without overlapping in any column?

Approach: for each (key, lock) pair, sum each of the pin heights and return false
if it exceeds the available space (5 in this example).
*/

use utils::{read_array_from_string, read_from_args};

fn main() {
    let input = read_from_args().unwrap();

    let mut locks: Vec<Vec<usize>> = Vec::new();
    let mut keys: Vec<Vec<usize>> = Vec::new();
    for schematic in input.split("\n\n") {
        if schematic.starts_with("###") {
            let lock = read_schematic(schematic, true);
            locks.push(lock)
        } else {
            let key = read_schematic(schematic, false);
            keys.push(key)
        }
    }

    let mut fit = 0;
    for lock in locks {
        for key in &keys {
            if key.iter().zip(&lock).all(|(k, l)| k + l <= 5) {
                fit += 1;
            }
        }
    }
    dbg!(fit);
}

fn read_schematic(schematic: &str, lock: bool) -> Vec<usize> {
    let schema_array = read_array_from_string(schematic.to_owned());
    let mut columns: Vec<usize> = Vec::new();
    for j in 0..schema_array[0].len() {
        let mut column = 0;
        for i in 0..schema_array.len() {
            if lock && i == 0 {
                continue;
            }
            if !lock && i == schema_array.len() - 1 {
                continue;
            }
            if schema_array[i][j] == '#' {
                column += 1;
            }
        }
        columns.push(column);
    }
    columns
}