/*
Part 1: you are given the map of the warehouse and a sequence of robot movements.
Robot is represented by @ and boxes are represented by O. Walls are represented by #.
Robot moves are represented by ^, v, <, >.
Pushing against boxes moves those boxes, pushing against a wall results in no movement.

########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<

The GPS coordinate of a box is given by 100 x distance from the top edge + distance from the left edge.
What is the sum of all boxes' GPS coordinates after the robot finishes moving?

Approach: the robot's movement depends on:
(1) input direction
(2) whether there's a wall in contact on that direction (accounting for any boxes in between)
(3) whether there's a box in contact on that direction

boxes_to_move = []
search_boxes(robot, direction, boxes_to_move)

fn search_boxes(position, direction, boxes_to_move) {
    if position is wall: boxes_to_move.empty(); return
    if position is empty space: return
    if position is box:
        boxes_to_move.append(position);
        search_boxes(position + direction, direction, boxes_to_move)
}

Part 2: everything is twice as wide, apart from the robot

##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############

The logic is still the same, but we need to account for the facts that the two parts of the box must move together,
and the wall-box and box-box collision has to detect partial collisions.

boxes_to_move = [] // this is just the left edges of each box
search_boxes(robot, direction, boxes_to_move)

fn move_robot(position, direction, boxes_to_move) {
    if position is wall: boxes_to_move.empty(); return false
    if position is empty space: return true
    if position is box:
        if position is left edge of box:
            left edge = position
        if position is right edge of box:
            left edge = position + left
        boxes_to_move.append(left edge)

        if direction is horizontal:
            search_boxes(position + 2*direction, direction, boxes_to_move)
        if direction is vertical:
            search_boxes(left edge + direction, direction, boxes_to_move)
            search_boxes(left edge + right + direction, direction, boxes_to_move)
}
*/
use std::{collections::HashSet, io};
use std::io::{stdin, stdout, Read, Write};

use utils::{add_direction, read_array_from_string, read_from_args, Direction, E, N, S, W};

const BOX_WIDTH: usize = 2;

fn main() -> io::Result<()> {
    let input = read_from_args()?;

    let (map_raw, input_directions) = input.split_once("\n\n").unwrap();
    let mut map = read_array_from_string(map_raw.to_string());
    if BOX_WIDTH == 2 {
        map = double_map(&map);
    }

    let (mut robot, mut boxes, walls) = parse_map(&map);

    input_directions.chars().for_each(|direction_symbol| {
        let direction: Direction = match direction_symbol {
            '^' => N,
            'v' => S,
            '<' => W, 
            '>' => E,
            '\n' => return,
            _ => panic!("Unexpected direction.")
        };

        let neighbor = add_direction(robot, direction);
        let mut boxes_to_move = vec![];

        // print_map(&map);
        // pause();

        if search_boxes(neighbor, direction, &mut boxes_to_move, &boxes, &map) {
            robot = add_direction(robot, direction);
            boxes_to_move.iter().for_each( |&b| {
                boxes[b] = add_direction(boxes[b], direction);
            });
        }

        update_map(&mut map, &robot, &boxes, &walls);
    });
    print_map(&map);
    let gps = boxes.iter().fold(0, 
        |total, b| {
            total + 100 * b.0 + b.1
        }
    );
    dbg!(gps);

    Ok(())
}

fn search_boxes(position: (usize, usize), direction: Direction, boxes_to_move: &mut Vec<usize>, boxes: &Vec<(usize, usize)>, map: &Vec<Vec<char>>, ) -> bool {    
    let object = map[position.0][position.1];
    match (direction, object) {
        (_, '#') => {
            *boxes_to_move = vec![]; // no boxes to move
            false
        },
        (_, '.') => {
            true
        },
        (_, 'O') => {
            let box_index = boxes.iter().position(|&b| b == position).unwrap(); 
            (*boxes_to_move).push(box_index);
            let neighbor = add_direction(position, direction);
            search_boxes(neighbor, direction, boxes_to_move, boxes, map)
        },
        (E | W, ']') => {
            let neighbor = add_direction(position, direction);
            search_boxes(neighbor, direction, boxes_to_move, boxes, map)
        },
        (N|S, ']') => {
            let left_edge = add_direction(position, W);
            search_boxes(left_edge, direction, boxes_to_move, boxes, map)
        }
        (_, '[') => {
            let box_index = boxes.iter().position(|&b| b == position).unwrap();
            if !boxes_to_move.contains(&box_index) {
                (*boxes_to_move).push(box_index);
            }

            if direction == E || direction == W { // horizontal
                let neighbor = add_direction(position, direction);
                search_boxes(neighbor, direction, boxes_to_move, boxes, map)
            } else {
                let neighbor_left = add_direction(position, direction);
                let right_edge = add_direction(position, E);
                let neighbor_right = add_direction(right_edge, direction);
                search_boxes(neighbor_left, direction, boxes_to_move, boxes, map)
                && search_boxes(neighbor_right, direction, boxes_to_move, boxes, map)
            }
        },
        _ => panic!("Unexpected object in map.")
    }
}

fn update_map(map: &mut Vec<Vec<char>>, robot: &(usize, usize), boxes: &Vec<(usize, usize)>, walls: &HashSet<(usize, usize)>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            map[i][j] = '.';
        }
    }

    map[robot.0][robot.1] = '@';

    for b in boxes {
        if BOX_WIDTH == 1 {
            map[b.0][b.1] = 'O';
        } else if BOX_WIDTH == 2 {
            map[b.0][b.1] = '[';
            map[b.0][b.1 + 1] = ']';
        }
    }

    for w in walls {
        map[w.0][w.1] = '#';
    }
}

fn parse_map(map: &Vec<Vec<char>>) -> ((usize, usize), Vec<(usize, usize)>, HashSet<(usize, usize)>) {
    let mut robot: (usize, usize) = (0, 0);
    let mut boxes: Vec<(usize, usize)> = Vec::new();
    let mut walls: HashSet<(usize, usize)> = HashSet::new();
    
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            match map[i][j] {
                '.'|']' => continue,
                '#' => {
                    walls.insert((i, j));
                },
                '[' => boxes.push((i, j)),
                'O' => boxes.push((i, j)),
                '@' => {robot = (i, j)},
                _ => panic!("Unexpected map object.")
            }
        }
    }
    (robot, boxes, walls)    
}

fn double_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut doubled_map: Vec<Vec<char>> = Vec::new();
    for i in 0..map.len() {
        let mut doubled_line: Vec<char> = Vec::new();
        for j in 0..map[0].len() {
            match map[i][j] {
                '.' => {
                    doubled_line.extend(['.', '.']);
                },
                '#' => {
                    doubled_line.extend(['#', '#']);
                },
                'O' => {
                    doubled_line.extend(['[', ']']);
                },
                '@' => {
                    doubled_line.extend(['@', '.']);
                },
                _ => panic!("Unexpected object in map")
            };
        }
        doubled_map.push(doubled_line);
    }
    doubled_map
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let object = map[i][j];
            print!("{object}");
        }
        println!();
    }
}
