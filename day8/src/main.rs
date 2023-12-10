use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

fn main() {
    let file_path = "input.txt";
    let map_input = fs::read_to_string(file_path).expect("Could not open file");

    let (instructions, network) = parse_map(&map_input);
    println!("{instructions}");
    // println!("{:?}", network);

    let mut current_node: &String = &String::from_str("AAA").unwrap();
    let mut current_instruction = 0;
    let mut steps = 0;
    while current_node != "ZZZ" {
        let left_right = network.get(current_node).unwrap();
        current_node = if instructions.chars().nth(current_instruction).unwrap() == 'L' {
            &left_right.0
        } else {
            &left_right.1
        };
        steps += 1;
        current_instruction = steps % instructions.len();
        // if steps % 1_000 == 0 {
        //     println!("Step: {steps}");
        // }
    }

    println!("Number of steps: {steps}")
}

fn parse_map(map_input: &str) -> (&str, HashMap<String, (String, String)>) {
    let instructions = map_input
        .lines()
        .next()
        .expect("First line should contain left/right instructions");

    let mut network: HashMap<String, (String, String)> = HashMap::new();
    for node in map_input.strip_prefix(instructions).unwrap().trim().lines() {
        let (element, left_right) = node.split_once(" = ").unwrap();
        let (left, right) = left_right.split_once(',').unwrap();

        let left = left.strip_prefix('(').unwrap().trim();
        let right = right.strip_suffix(')').unwrap().trim();

        network.insert(element.to_string(), (left.to_string(), right.to_string()));
    }
    (instructions, network)
}
