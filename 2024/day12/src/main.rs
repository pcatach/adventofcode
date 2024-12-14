/*
Part 1: given a map of garden plots
AAAA
BBCD
BBCC
EEEC

A region's area is the number of garden plots,
e.g. region A has area 4.
A region's perimeter is the number of sides of garden plots
in the region that do not touch another garden plot in the same region.
Region A has perimeter 10.

The price of fence for a region is the product of its area and its perimeter.
In the first example above, price(A) = 4*10, price(B) = 4*8, price(C) = 4*10, 
price(D) = 1*4, price(E) = 3*8.
What is the total price?

Approach: 
Find connected clusters of plots of the same type.
For each plot:
    If plot is already in cluster, return
    If cluster is empty or if plot is of the same type as cluster, add plot
    Recurse on each neighbor

Area will be given by number of plots in the cluster, perimeter can be computed
on the go by adding 1 for each "wall" that you hit 

Part 2: price is given by area * number of sides instead of perimeter.
Number of sides is the same as the number of corners

A A A A
-------
B B |C| D
B B |C  C
------
E E E | C

*/
use std::io;

use utils::{read_from_args, read_array_from_string};

#[derive(Debug, Clone)]
struct Cluster {
    cluster_type: char,
    perimeter: usize,
    sides: usize,
    plots: Vec<(usize, usize)>
}

fn main() -> io::Result<()> {
    let map = read_array_from_string(read_from_args()?);
    // dbg!(&map);

    let clusters = find_all_clusters(&map);
    // dbg!(&clusters);

    let price = clusters.iter().fold(0, |acc, cluster| 
        acc + cluster.plots.len() * cluster.perimeter
    );
    dbg!(price);
    Ok(())
}

fn find_all_clusters(
    map: &Vec<Vec<char>>,
) -> Vec<Cluster> {
    let height = map.len();
    let width = map[0].len();

    let mut clusters: Vec<Cluster> = Vec::new();
    for i in 0..height {
        for j in 0..width {
            if clusters.iter().any(|cluster| cluster.plots.contains(&(i,j))) {
                continue;
            }
            let mut new_cluster = Cluster {
                cluster_type: map[i][j],
                perimeter: 0,
                sides: 0,
                plots: Vec::new()
            };
            dbg!(map[i][j]);
            find_cluster(&map, height, width, &mut new_cluster, (i, j));
            dbg!(new_cluster.sides);
            clusters.push(new_cluster);
        }
    }
    clusters
}

fn find_cluster(
    map: &Vec<Vec<char>>, 
    height: usize,
    width: usize,
    cluster: &mut Cluster,
    plot: (usize, usize)
) {
    cluster.plots.push(plot);

    let mut walls: Vec<usize> = Vec::new();
    for (i, neighbor) in get_neighbors(plot, map).iter().enumerate() {
        match neighbor {
            None => {
                cluster.perimeter += 1;
                walls.push(i);
            },
            Some(n) => {
                if cluster.cluster_type != map[n.0][n.1] {
                    cluster.perimeter += 1;
                    walls.push(i);
                } else if !cluster.plots.contains(&n) {
                    find_cluster(map, height, width, cluster, *n);
                }
            }
        }        
    }
    if walls.len() == 2 {
        if (walls[1] - walls[0]) % 2 == 1 {
            cluster.sides += 2;
        }
    } else if walls.len() == 3 {
        cluster.sides += 3;
    } else if walls.len() == 4 {
        cluster.sides += 4
    }
}

fn get_neighbors(plot: (usize, usize), map: &Vec<Vec<char>>) -> Vec<Option<(usize, usize)>> {
    let height = map.len();
    let width = map[0].len();

    [
        (-1, 0), // up
        (0, 1), // right
        (1, 0), // down
        (0, -1) // left
    ].iter().map(|&(di, dj)| {
        map_add(plot, (di, dj), height, width)
    }).collect()
}

fn map_add(
    (i, j): (usize, usize),
    (di, dj): (isize, isize), 
    height: usize, 
    width: usize
) -> Option<(usize, usize)> {
    let (ni, nj) = (i as isize + di, j as isize + dj);
    if ni >= 0 && nj >= 0 && ni < height as isize && nj < width as isize {
        return Some((ni as usize, nj as usize));
    };
    None
}