use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let tiles_input = fs::read_to_string(file_path).expect("Could not read file");
    let tiles = parse_tiles(&tiles_input);

    let (&start, _) = tiles.iter().find(|&(_, tile)| *tile == 'S').unwrap();

    let mut path: Vec<(i32, i32)> = Vec::new();
    path.push(start);
    // find first step
    let mut current: (i32, i32) = start;
    for dir in [(0, 1), (-1, 0), (1, 0), (0, -1)] {
        let empty_path: Vec<(i32, i32)> = Vec::new();
        if step(add(start, dir), &empty_path, &tiles).is_some() {
            current = add(start, dir);
        }
    }
    path.push(current);

    while *tiles.get(&current).unwrap() != 'S' {
        // println!(
        //     "current tile: {:?}, {}, path: {:?}",
        //     current,
        //     tiles.get(&current).unwrap(),
        //     path
        // );
        current = match step(current, &path, &tiles) {
            Some(c) => c,
            None => break,
        };
        path.push(current);
    }
    println!("Path length: {}", path.len());
    println!(
        "Steps to point farthest from S: {:?}",
        (path.len() as f64) / 2.0
    );

    let height = tiles_input.lines().count();
    let width = tiles_input.lines().next().unwrap().chars().count();
    let mut area = 0;
    for i in 0..height {
        for j in 0..width {
            if is_inside_loop((i as i32, j as i32), &path, &tiles) {
                area += 1;
                // println!("({i},{j}) is inside loop");
            }
        }
    }
    println!("Area enclosed by the loop is {area}");
}

fn is_inside_loop(
    tile: (i32, i32),
    path: &Vec<(i32, i32)>,
    tiles: &HashMap<(i32, i32), char>,
) -> bool {
    // ray casting algorithm
    if path.contains(&tile) {
        return false;
    }
    let mut num_crosses = 0;
    for j in 0..tile.1 {
        if path.contains(&(tile.0, j)) && ['|', 'L', 'J'].contains(tiles.get(&(tile.0, j)).unwrap())
        {
            num_crosses += 1;
        }
    }

    num_crosses % 2 != 0
}

fn step(
    current: (i32, i32),
    path: &Vec<(i32, i32)>,
    tiles: &HashMap<(i32, i32), char>,
) -> Option<(i32, i32)> {
    match tiles.get(&current) {
        Some('F') if !path.contains(&add(current, (0, 1))) => Some(add(current, (0, 1))),
        Some('F') if !path.contains(&add(current, (1, 0))) => Some(add(current, (1, 0))),
        Some('7') if !path.contains(&add(current, (0, -1))) => Some(add(current, (0, -1))),
        Some('7') if !path.contains(&add(current, (1, 0))) => Some(add(current, (1, 0))),
        Some('|') if !path.contains(&add(current, (-1, 0))) => Some(add(current, (-1, 0))),
        Some('|') if !path.contains(&add(current, (1, 0))) => Some(add(current, (1, 0))),
        Some('J') if !path.contains(&add(current, (0, -1))) => Some(add(current, (0, -1))),
        Some('J') if !path.contains(&add(current, (-1, 0))) => Some(add(current, (-1, 0))),
        Some('L') if !path.contains(&add(current, (0, 1))) => Some(add(current, (0, 1))),
        Some('L') if !path.contains(&add(current, (-1, 0))) => Some(add(current, (-1, 0))),
        Some('-') if !path.contains(&add(current, (0, -1))) => Some(add(current, (0, -1))),
        Some('-') if !path.contains(&add(current, (0, 1))) => Some(add(current, (0, 1))),
        _ => None,
    }
}
fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}
fn parse_tiles(tiles_input: &str) -> HashMap<(i32, i32), char> {
    let mut tiles: HashMap<(i32, i32), char> = HashMap::new();
    for (i, line) in tiles_input.lines().enumerate() {
        for (j, tile) in line.chars().enumerate() {
            tiles.insert((i as i32, j as i32), tile);
        }
    }
    tiles
}

// fn get_available_directions(tile: Tile, tiles: Vec<Tile>) {
//     for direction in [(0,1), (1,0), (0, -1), (-1, 0)] {
//         let new_tile = (tile.1, tile.2);
//         if new_tile + direction > (0,0) and new_tile + direction < (tiles.)
//     }
// }
