/*
Part 1: map represents a location of antennas and their frequencies
(0-9 a-z A-Z).
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
An antinode occurs at any point in line with two antennas of the same
frequency, when one of them is twice as far as the other.
...#......
#.........
....a.....
........a.
.....a....
..#.......
......#...
..........
In the case above, the antinode at (0, 3) is at distance 3 of antenna "a"
at (2, 4) and distance 6 of antenna (4, 5). The distance between the antennas
is 3
How many unique locations within the bounds of the map contain an antinode?

Approach: get all pairs of antennas x, y with the same frequency. Compute distance
d between them. Antinode will be at points n given by
(2x - y), (2y - x), (x + 2y) / 3, (2x + y) / 3
Be careful to check if it's within map and if it's in the grid

Part 2: extra antinodes created by "resonant harmonics"
any grid point in line with at least two antennas of the same frequency
Given two points p1, p2, how do you find all points in the same line?
The line direction is (p2 - p1), and it can be parametrized as
p = p1 + n*(p2 - p1)/||p2-p1||
*/
use std::{cmp, collections::HashSet, io};

use utils::{read_array_from_string, read_from_args};

use cgmath::Vector2;

type Antenna = (Vector2<f64>, char);

fn main() -> io::Result<()> {
    let map = read_array_from_string(read_from_args()?);
    let height = map.len();
    let width = map[0].len();

    let antennas = get_antennas(map, height, width);

    let mut distinct_antinodes: HashSet<Vector2<usize>>= HashSet::new();
    for (i, antenna_i) in antennas.iter().enumerate() {
        for antenna_j in antennas.iter().take(i) {
            if antenna_i.1 == antenna_j.1 {
                // dbg!(antenna_i, antenna_j);
                let antinodes = get_antinodes(antenna_i.0, antenna_j.0, height, width);
                // dbg!(&antinodes);
                for antinode in antinodes.iter() {
                    distinct_antinodes.insert(antinode.to_owned());
                }
            }
        }
    }

    dbg!(distinct_antinodes.len());

    Ok(())
}

fn get_antennas(map: Vec<Vec<char>>, height: usize, width: usize) -> Vec<Antenna> {
    let mut antennas: Vec<Antenna> = Vec::new();
    for i in 0..height {
        for j in 0..width {
            match map[i][j] {
                '.' => (),
                'A'..='Z' |'a'..='z' | '0'..='9' => {
                    antennas.push((Vector2::new(i as f64, j as f64), map[i][j]));
                },
                _ => panic!("Cannot read map.")
            }
        }
    }
    antennas
}

fn get_antinodes(pos_1: Vector2<f64>, pos_2: Vector2<f64>, height: usize, width: usize) -> Vec<Vector2<usize>> {
    // let antinode_positions =     [
    //     2.0*pos_1 - pos_2,
    //     2.0*pos_2 - pos_1,
    //     (pos_1 + 2.0*pos_2) / 3.0,
    //     (pos_2 + 2.0*pos_1) / 3.0,
    // ];

    let direction = pos_2 - pos_1;
    let max_n = cmp::max(width, height) as isize;

    let antinode_positions = (-max_n..=max_n).map(
        |n| pos_1 + (n as f64) * direction
    );

    antinode_positions
    .filter_map(
        |a| match 
            a[0] >= 0.0 && a[0] < height as f64
            && a[1] >= 0.0 && a[1] < width as f64
            && a[0] == a[0].floor() && a[1] == a[1].floor()
        {
            true => Some(Vector2::new(a[0].floor() as usize, a[1].floor() as usize)),
            false => None
        }
    ).collect()
}