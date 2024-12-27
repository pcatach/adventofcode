/*
Part 1: give boolean logic gates:
- AND a b = a & b
- OR a b = a | b
- XOR a b = a ^ b
you are given initial wire values and gate connections:
```
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
```
The system outputs a binary number formed by wires starting with z, in this case:
```
z00 = 0
z01 = 0
z02 = 1
```
which forms b100 = 4.

Approach: load the x,y values into a hash table, and the logic gates
into another hash table, and define a recursive function for the simulation
*/

use std::collections::HashMap;

use utils::read_from_args;

fn main() {
    let input = read_from_args().unwrap();
    
    let mut initial: HashMap<&str, usize> = HashMap::new();
    let mut gates: HashMap<&str, (&str, &str, &str)> = HashMap::new();
    
    let mut lines = input.lines();
    let mut read_gates = false;
    while let Some(line) = lines.next() {
        if line == "" {
            read_gates = true;
            continue;
        }
        if read_gates {
            let (rest, wire) = line.split_once(" -> ").unwrap();
            let mut split = rest.splitn(3, " ");
            let (w1, op, w2) = 
                (split.next().unwrap(), split.next().unwrap(), split.next().unwrap());
            gates.insert(wire, (op, w1, w2));
            continue;
            }
        let (wire, value) = line.split_once(": ").unwrap();
        initial.insert(wire, value.parse::<usize>().unwrap());
    }

    let mut output: String = String::new();
    let mut zgates = gates.clone().into_keys()
        .filter(|&g| g.starts_with("z"))
        .collect::<Vec<&str>>();
    zgates.sort();
    for &gate in zgates.iter().rev() {
        let value = simulate(&gate, &initial, &gates);
        output.push_str(&value.to_string());
    }
    dbg!(&output);
    dbg!(usize::from_str_radix(&output, 2).unwrap());
}

fn simulate(gate: &str, initial: &HashMap<&str, usize>, gates: &HashMap<&str, (&str, &str, &str)>) -> usize {
    if let Some(&value) = initial.get(gate) {
        return value;
    } 
    let (op, w1, w2) = gates.get(gate).unwrap();
    let v1 = simulate(w1, initial, gates);
    let v2 = simulate(w2, initial, gates);
    match *op {
        "OR" => v1 | v2,
        "AND" => v1 & v2,
        "XOR" => v1 ^ v2,
        _ => unreachable!()
    }
}
