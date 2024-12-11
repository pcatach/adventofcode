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
in the map and h(T) - h(S) = 1.
Do a DFS from each 0, count the number of 9s it can reach
*/
use std::{collections::{HashMap, HashSet}, io};

use utils::{read_from_args, read_array_of_numbers_from_string};

fn main() -> io::Result<()> {
    let map_input = read_from_args()?;
    let topographic_map: Vec<Vec<u32>> = read_array_of_numbers_from_string(map_input);
    let height = topographic_map.len();
    let width = topographic_map[0].len();
    // dbg!(&topographic_map);

    let graph = build_graph(&topographic_map, height, width);
    // dbg!(&graph);

    let mut hiking_trails_per_head: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    
    for starting_node in graph.keys() {
        if topographic_map[(*starting_node).0][(*starting_node).1] == 0 {
            let mut hiking_trails: Vec<(usize, usize)> = Vec::new();
            dfs(
                &topographic_map,
                &graph,
                starting_node,
                &mut hiking_trails,
            );
            hiking_trails_per_head.insert(*starting_node, hiking_trails);
        }
    }

    let sum_of_scores = hiking_trails_per_head.values()
    .map(|s| s.iter().cloned().collect::<HashSet<(usize, usize)>>().len())
    .sum::<usize>();

    let sum_of_ratings = hiking_trails_per_head.values()
    .map(Vec::len)
    .sum::<usize>();

    dbg!(sum_of_scores);
    dbg!(sum_of_ratings);
    Ok(())
}

fn dfs(
    map: &Vec<Vec<u32>>,
    graph: &HashMap<(usize, usize), Vec<(usize, usize)>>, 
    node: &(usize, usize), 
    hiking_trails: &mut Vec<(usize, usize)>,
) {
    if map[node.0][node.1] == 9 {
        hiking_trails.push(*node);
    };

    let edges = match graph.get(node) {
        Some(edges) => edges,
        _ => return
    };

    for neighbor in edges {
        dfs(map, graph, neighbor, hiking_trails);
    }
}

fn build_graph(
    map: &Vec<Vec<u32>>, height: usize, width: usize
) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let mut graph: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for i in 0..height {
        for j in 0..width {
            let node = map[i][j];
            for potential_neighbor in [
                (i.checked_sub(1), Some(j)), // up
                (Some(i+1).filter(|&r| r < height), Some(j)), // down
                (Some(i), j.checked_sub(1)), // left 
                (Some(i), Some(j+1).filter(|&r| r < width)), // right
            ] {
                let neighbor_pos = match potential_neighbor {
                    (Some(ni), Some(nj)) => (ni, nj),
                    _ => continue
                };

                let neighbor = map[neighbor_pos.0][neighbor_pos.1];
                if neighbor.saturating_sub(node) == 1 {
                    match graph.get_mut(&(i,j)) {
                        Some(edges) => edges.push(neighbor_pos),
                        None => {
                            let mut edges = Vec::new();
                            edges.push(neighbor_pos);
                            graph.insert((i, j), edges);
                        }
                    }
                }
            }
        }
    }
    graph
}