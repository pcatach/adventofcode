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

Part 2: the circuit is trying to add numbers x and y via bitwise addition
of x00 and y00 (least significant), x01 and y01, and so on. So it should take two
n-bit numbers x and y and produce a (n+1)-bit number z.
But there are 4 pairs of gates whose outputs have been swapped. Which four pairs (so 8 wires)
need to be swapped so that the circuit performs addition?

Approach: how many such pairs can exist? Assume we have N gates. We're looking for
4 pairs of 2 gates to swap. As there are 222 gates the total number of 4 pair combinations is
222! / [(222 - 8)! 2^2 * 3 ] ~ 1e12

Addition is implemented using a ripple-carry adder. If z = x + y, the nth bit of z will be
z_n = a_n XOR b_n XOR c_n
where
c_{n+1} = (a_n AND b_n) OR (c_n AND (a_n XOR b_n))
and c_0 = 0

We know that bit 6 is wrong: it does 1 + 0 = 1 but then it does 1 + 1 = 100.
So it's failing to add the carry correctly. It does:
z06 = kbj XOR dhs (kbj is x06 XOR y06 so we know dhs is the carry c_6)
z07 = x07 AND y07 (is wrong)
when it should be
z07 = x07 XOR y07 XOR ((x06 AND y06) OR (c6 AND (x06 XOR y06)))
replacing the gates:
z07 = pkm XOR (qrw OR (dhs AND kbj))
z07 = pkm XOR (qrw OR rmq) = pkm OR cvq
So we need to swap z07 with vmv

Now onto bit 19: it does 0 + 1 = 1 but 1 + 1 = 100
z20 = jbq OR hds (is wrong)
hds = x20 AND y20 so should be ORed with jbq to give the carry c21
but we have instead
z21 = jnb XOR kfm
as jnb = x21 OR y21, kfm is the carry so we should swap z20 with kfm

Now onto bit 27. It does 0 + 1 fine but 1 + 1 = 100
z28 = vwh AND jsg (is wrong)
vwh = x28 XOR y28
where we have
hnv = jsg XOR vwh
so we swap z28 with hnv

Onto bit 35. Now it does 0 + 1 = 10 and 1 + 1 = 1.
z36 = vvp XOR dkj = (x36 XOR y36) XOR dkj
dkj = hth OR chh = (x35 XOR y35) OR (vkc AND tqr) = (x35 XOR y35) OR ((npt OR krf) AND (x35 AND y35))
so looks like hth and tqr are swapped.

Result: hnv,hth,kfm,tqr,vmv,z07,z20,z28
*/

use std::collections::HashMap;

use utils::{pause, read_from_args};

fn main() {
    let input = read_from_args().unwrap();
    
    let (mut initial, mut gates) = parse_input(&input);
    let n = initial.len() / 2 + 1;
    let output = simulate_circuit(n, &initial, &gates);
    dbg!(n, output);

    for bit_to_check in 0..n - 1 {
        for (x, y) in [(0, 1), (1, 1)] {
            let (x_shifted, y_shifted) = (x << bit_to_check, y << bit_to_check);
            fill_input(&mut initial, x_shifted, y_shifted, n);
            let output = simulate_circuit(n, &initial, &gates);
            let different = output != x_shifted + y_shifted;
            println!("Bit {bit_to_check}: {x} + {y} = {:b}, {}", output >> bit_to_check, if different {"!!"} else {""});
            if different {
                pause();
            }
        }
    }

}

fn simulate_circuit(n: usize, initial: &HashMap<String, usize>, gates: &HashMap<&str, (&str, &str, &str)>) -> usize {
    let mut output: String = String::new();

    let zgates = (0..n).rev().map(|i| format!("z{i:0>2}"));
    for gate in zgates {
        let value = simulate(&gate, &initial, &gates);
        output.push_str(&value.to_string());
    }
    return usize::from_str_radix(&output, 2).unwrap();
}

fn simulate(gate: &str, initial: &HashMap<String, usize>, gates: &HashMap<&str, (&str, &str, &str)>) -> usize {
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

/// Take an usize like 2, convert it to b10 and fill x00=0, x01=1.
fn fill_input(initial: &mut HashMap<String, usize>, mut x: usize, mut y: usize, maxbit: usize) {
    let mut bit = 0;
    while bit < maxbit {
        let (xbit, ybit) = (x % 2, y % 2);
        let (xname, yname) = (format!("x{bit:0>2}"), format!("y{bit:0>2}"));
        initial.insert(xname, xbit);
        initial.insert(yname, ybit);
        (x, y, bit) = (x / 2, y / 2, bit + 1);
    }
}

fn parse_input(input: &str) -> (HashMap<String, usize>, HashMap<&str, (&str, &str, &str)>) {
    let mut initial: HashMap<String, usize> = HashMap::new();
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
        initial.insert(wire.to_owned(), value.parse::<usize>().unwrap());
    }
    (initial, gates)
}