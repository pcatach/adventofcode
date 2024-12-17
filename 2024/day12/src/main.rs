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

Part 2: price is given by the product of a region's area and it's number of sides.
*/
use std::io;

use utils::{
    Direction,
    DIRECTIONS4, DIRECTIONS8,
    read_from_args, read_array_from_string
};

#[derive(Debug)]
struct Cluster {
    cluster_type: char,
    perimeter: usize,
    corners: usize,
    plots: Vec<(usize, usize)>
}

fn main() -> io::Result<()> {
    let map = read_array_from_string(read_from_args()?);
    // dbg!(&map);

    let clusters = find_all_clusters(&map);
    // dbg!(&clusters);

    let price = clusters.iter().fold((0, 0), |acc, cluster| {
        let area = cluster.plots.len();
        (acc.0 + area * cluster.perimeter, acc.1 + area * cluster.corners)
    });
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
            // if plot already in one of the clusters, ignore
            if clusters.iter().any(|cluster| cluster.plots.contains(&(i,j))) {
                continue;
            }

            // create new cluster
            let mut new_cluster = Cluster {
                cluster_type: map[i][j],
                perimeter: 0,
                corners: 0,
                plots: Vec::new()
            };
            find_cluster(&map, height, width, &mut new_cluster, (i, j));
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
    // depth-first search
    process_plot(map, height, width, cluster, plot);
    cluster.plots.push(plot);

    for direction in DIRECTIONS4 {
        let Some(neighbor) = map_add(plot, direction, height, width) else {
            continue;
        };

        if map[plot.0][plot.1] == map[neighbor.0][neighbor.1]
        && !cluster.plots.contains(&neighbor) {
            find_cluster(map, height, width, cluster, neighbor);
        }
    }
}

fn process_plot(
    map: &Vec<Vec<char>>,
    height: usize, 
    width: usize, 
    cluster: &mut Cluster, 
    plot: (usize, usize)
){
    for i in (0..DIRECTIONS8.len()).step_by(2) {
        let directions: Vec<&Direction> = DIRECTIONS8.iter().cycle().skip(i).take(3).collect();

        // if up/right/down/left is blocked, add 1 to perimeter
        let blocked = map_add(plot, *directions[0], height, width)
            .map_or(true, |(x, y)| map[x][y] != cluster.cluster_type);
        cluster.perimeter += blocked as usize;

        // clunky corner detection
        let corner_state: Vec<bool> = directions.iter().map(|&dir|
            map_add(plot, *dir, height, width)
                .map_or(true, |(x, y)| map[x][y] != cluster.cluster_type)
        ).take(3).collect();

        if matches!(
            [corner_state[0], corner_state[1], corner_state[2]], 
            [true, true, true] // outer corner
            | [false, true, false] // inner corner
            | [true, false, true] // special case inner corner
        ) {
            
            cluster.corners += 1;
        }
    }
}

fn map_add(
    (i, j): (usize, usize),
    direction: Direction,
    height: usize, 
    width: usize
) -> Option<(usize, usize)> {
    let (ni, nj) = (i as isize + direction.x, j as isize + direction.y);
    if ni >= 0 && nj >= 0 && ni < height as isize && nj < width as isize {
        return Some((ni as usize, nj as usize));
    };
    None
}