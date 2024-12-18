use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::ops::Neg;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Direction {
    pub x: isize,
    pub y: isize
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Direction {
            x: -self.x,
            y: -self.y
        }
    }
}

pub const N: Direction  = Direction { x: -1, y: 0 };
pub const NE: Direction = Direction { x: -1, y: 1 };
pub const E: Direction = Direction { x: 0, y: 1 };
pub const SE: Direction = Direction { x: 1, y: 1 };
pub const S: Direction = Direction { x: 1, y: 0 };
pub const SW: Direction = Direction { x: 1, y: -1 };
pub const W: Direction = Direction { x: 0, y: -1 };
pub const NW: Direction = Direction { x: -1, y: -1 };
pub const DIRECTIONS4: [Direction; 4] = [N, E, S, W];
pub const DIRECTIONS8: [Direction; 8] = [N, NE, E, SE, S, SW, W, NW];

pub fn read_from_args() -> io::Result<String> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    fs::read_to_string(file_path)
}

pub fn read_array_from_string(text: String) -> Vec<Vec<char>> {
    text.lines().map(
        |line| line.chars().collect::<Vec<char>>()
    ).collect()
}

pub fn read_array_of_numbers_from_string(text: String) -> Vec<Vec<u32>> {
    text.lines().map(
        |line| line.chars()
        .map(
            |c| if c == '.' {100} else {c.to_digit(10).unwrap()}
        )
        .collect::<Vec<u32>>()
    ).collect()
}

pub fn add_direction(position: (usize, usize), direction: Direction) -> (usize, usize) {
    (
        position.0.saturating_add_signed(direction.x),
        position.1.saturating_add_signed(direction.y)
    )
}

pub fn add_checked_direction(position: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
    let opt_x = position.0.checked_add_signed(direction.x);
    let opt_y = position.1.checked_add_signed(direction.y);

    match (opt_x, opt_y) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None
    }
}

pub fn pause() {
    let mut stdout = io::stdout();
    stdout.flush().unwrap();
    io::stdin().read(&mut [0]).unwrap();
}