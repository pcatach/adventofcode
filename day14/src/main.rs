use memoize::memoize;
use std::fs;

fn main() {
    let file_path = "example.txt";
    let mut platform = fs::read_to_string(file_path).expect("Unable to read file");
    let tilted_north = rotate(&rotate(&rotate(&platform)));
    let tilted_north = tilt_left(&tilted_north);
    let tilted_north = rotate(&tilted_north);
    let load = compute_load_on_north(&tilted_north);
    println!("Total load after 1st north tilt: {load}");
    println!("Tilted platform:\n{tilted_north}");

    let cycles = 1000;
    // need to position the north section to the left initially
    platform = rotate(&rotate(&rotate(&platform)));
    for i in 0..cycles {
        let cycled_platform = spin_cycle(&platform);
        platform = cycled_platform
    }
    // reposition north-north
    platform = rotate(&platform);
    println!("Platform after {cycles} cycles:\n{platform}");
    println!("Load on north beam is {}", compute_load_on_north(&platform))
}

#[memoize]
fn spin_cycle(platform: &str) -> String {
    // tilts left, rotate clockwise, repeats 3x
    let cycled = tilt_left(platform);
    let cycled = rotate(&cycled);
    let cycled = tilt_left(&cycled);
    let cycled = rotate(&cycled);
    let cycled = tilt_left(&cycled);
    let cycled = rotate(&cycled);
    let cycled = tilt_left(&cycled);
    rotate(&cycled)
}

fn tilt_left(platform: &str) -> String {
    let mut tilted_platform: String = String::new();
    for line in platform.lines() {
        let mut move_to = 0;
        let mut i = 0;
        while i < line.len() {
            let value = line.chars().nth(i).unwrap();
            match value {
                '.' => (),
                '#' => {
                    tilted_platform.push_str(&".".repeat(i.checked_sub(move_to).unwrap()));
                    tilted_platform.push('#');
                    move_to = i + 1
                }
                'O' => {
                    tilted_platform.push('O');
                    move_to += 1
                }
                _ => panic!("WHAT?"),
            }
            i += 1
        }
        tilted_platform.push_str(&".".repeat(i.checked_sub(move_to).unwrap()));
        tilted_platform.push('\n')
    }
    tilted_platform
}

fn compute_load_on_north(platform: &str) -> usize {
    let height = platform.lines().count();
    platform
        .lines()
        .enumerate()
        .map(|(i, line)| (height - i) * line.chars().filter(|c| *c == 'O').count())
        .sum()
}

// rotates platform clockwise
fn rotate(platform: &str) -> String {
    let height = platform.lines().count();
    let mut rotated_platform: String = String::new();
    for (i, _) in platform.lines().enumerate() {
        for (j, _) in platform.lines().next().unwrap().chars().enumerate() {
            rotated_platform.push(get_char_at(platform, height.checked_sub(j + 1).unwrap(), i))
        }
        rotated_platform.push('\n')
    }
    rotated_platform
}

fn get_char_at(platform: &str, i: usize, j: usize) -> char {
    let col_size = platform.lines().next().unwrap().len() + 1;
    platform.chars().nth(i * col_size + j).unwrap()
}
