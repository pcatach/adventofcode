/*
Part 1: you are given a list of (bidirectional) connections between two computers:
```
kh-tc
qp-kh
tc-qp
ka-co
yn-aq
qp-ub
```
Find all sets of 3 connected computers:
```
kh,tc,kp
```
How many of these contain at least one computer that starts with "t"?

Approach: the list gives you a graph, you can do DFS to find cycles of size 3

Part 2: What is the largest set of computers that are all connected to each other?
Every computer in that set must be connected to every other computer in the set.
*/

use std::collections::{HashMap, HashSet};

use utils::read_from_args;

const CYCLE_LENGTH: usize = 3;

fn main() {
    let input = read_from_args().unwrap();
    let mut connections = HashMap::new();
    input.lines()
        .map(|l| l.split_once("-").unwrap())
        .for_each(|connection| {
            connections.entry(connection.0).or_insert(vec![]).push(connection.1);
            connections.entry(connection.1).or_insert(vec![]).push(connection.0);
        });
    let computers = connections.keys().into_iter().map(|s| s.to_owned()).collect::<HashSet<&str>>();

    let mut stack: Vec<(&str, Vec<&str>)> = Vec::new();
    let mut cycles: HashSet<Vec<&str>> = HashSet::new();

    let mut computers_iter = computers.iter();
    while let Some(&start_computer) = computers_iter.next() {
        stack.push((&start_computer, vec![]));

        while let Some((computer, path)) = stack.pop() {
            let edges = connections.get(computer).unwrap();
                
            // find cycles of length CYCLE_LENGTH
            if start_computer.starts_with('t') && computer == start_computer && path.len() == CYCLE_LENGTH {
                cycles.insert(path.clone());
            }

            // search each edge up to depth CYCLE_LENGTH
            if path.len() < CYCLE_LENGTH {
                for &c in edges {
                    let mut new_path = path.clone();
                    new_path.push(c);
                    new_path.sort();
                    stack.push((c, new_path));
                }
            }
        }
        stack.clear();
    }
    dbg!(cycles.len());

    let mut fully_connected_sets: Vec<HashSet<&str>> = computers.iter().map(|&s| HashSet::from([s])).collect();
    for set in fully_connected_sets.iter_mut() {
        for &c in computers.iter() {
            // if all computers in set are connected to c, add c to set
            let edges = connections.get(c).unwrap();
            if set.iter().all(|s| edges.contains(s)) {
                set.insert(c);
            }
        }
    }

    let max_set = fully_connected_sets.iter().max_by_key(|s| s.len()).unwrap();
    let mut max_set_vec: Vec<&str> = max_set.iter().map(|s| s.to_owned()).collect();
    max_set_vec.sort();
    dbg!(max_set_vec.join(","));
}
