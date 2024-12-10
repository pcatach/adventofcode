/*
Part 1: you are given a topographic map with the height (0-9) at each position:
0123
1234
8765
9876

A hiking trail is any path that starts at height 0 and ends at height 9, and in between only take steps
up, down, left or right, always *increasing* at intervals of height 1.
A trailhead is any position that starts 1+ hiking trails (so will be at height 0).
Trailhead score is the number of hiking trails from that trailhead (so the number of 9s reachable from that 0)
Get the sum of the scores of all trailheads.

Approach: build a directed graph where node S has an edge to node T if they are adjacent points
in the map and h(S) - h(T) = 1.
Assume it is a directed acyclic graph.
Assume there's always at least one path between 0 and 9.
Count the number of paths between 0 and 9 with DFS
*/
use std::io;

use utils::{read_from_args, read_array_from_string};

fn main() -> io::Result<()> {
    let map_input = read_from_args()?;
    let topographic_map = read_array_from_string(map_input);
    dbg!(topographic_map);
    Ok(())
}