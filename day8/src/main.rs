use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "input.txt";
    let map_input = fs::read_to_string(file_path).expect("Could not open file");

    let (instructions, network) = parse_map(&map_input);
    println!("{instructions}");
    // println!("{:?}", network );

    // let mut current_node: &str = "AAA";
    let mut current_nodes: Vec<&str> = network
        .keys()
        .filter(|&x| (*x).ends_with('A'))
        .cloned()
        .collect();
    let mut current_instruction = 0;
    let mut steps = 0;
    let mut steps_for_each_path: Vec<usize> = Vec::new();

    // while current_node != "ZZZ" {
    // while !current_nodes.iter().all(|&node| node.ends_with('Z')) {
    while !current_nodes.is_empty() {
        // let left_right = network.get(current_node).unwrap();

        // println!("{:?}", current_nodes);
        let left_right: Vec<(&str, &str)> = current_nodes
            .iter()
            .map(|&node| *network.get(node).unwrap())
            .collect();
        current_nodes = if instructions.chars().nth(current_instruction).unwrap() == 'L' {
            // &left_right.0
            left_right.iter().map(|x| x.0).collect()
        } else {
            left_right.iter().map(|x| x.1).collect()
        };
        steps += 1;

        let drop_finished: Vec<&str> = current_nodes
            .iter()
            .filter(|&x| !(*x).ends_with('Z'))
            .cloned()
            .collect();

        for _ in 0..(current_nodes.len() - drop_finished.len()) {
            steps_for_each_path.push(steps);
        }

        current_nodes = drop_finished;
        current_instruction = steps % instructions.len();
    }
    let least_common_factor = steps_for_each_path
        .iter()
        .copied()
        .reduce(lcm_of_two_numbers)
        .unwrap();
    println!("Number of steps: {least_common_factor}")
}

fn parse_map(map_input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let instructions = map_input
        .lines()
        .next()
        .expect("First line should contain left/right instructions");

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    for node in map_input.strip_prefix(instructions).unwrap().trim().lines() {
        let (element, left_right) = node.split_once(" = ").unwrap();
        let (left, right) = left_right.split_once(',').unwrap();

        let left = left.strip_prefix('(').unwrap().trim();
        let right = right.strip_suffix(')').unwrap().trim();

        network.insert(element, (left, right));
    }
    (instructions, network)
}

pub fn lcm_of_two_numbers(a: usize, b: usize) -> usize {
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
