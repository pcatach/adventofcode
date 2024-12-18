/*
Part 1: You are given a program (sequence of numbers between 0 and 7), an instruction pointer initialised to 0 and 3 registers.
Each pair of numbers in the program represents an opcode and an operand.
The program halts when the instruction pointer goes past the end of the program.
Instructions can write to an output buffer.

There are 8 different opcodes and the instructions pointer increases by 2 after each instruction (apart from jump instructions).

Operands can be interpreted as literals (their own value) or mapped as combo operands.
instructions:

0 => adv: A = A / 2^COMBO(X)
1 => bxl: B = B ^ X
2 => bst: B = COMBO(X) % 8
3 => jnz: jump X if A != 0
4 => bxc: B = B ^ C
5 => out: print COMBO(X) % 8
6 => bdv: B = A / 2^COMBO(X)
7 => cdv: C = A / 2^COMBO(X)

COMBO(X) = {
    X if X<=3,
    A if X=4,
    B if X=5,
    C if X=6
}

Part 2: which initial value for register A causes the program to output a copy of itself?

Approach: let's assume the program is always a while loop on A>0 (always ends with 3, 0).
For any program like this to halt, A has to be modified and the only instruction that does this
is adv. Let's further assume that adv is always called with operand 3 (that's the case for
the second example and for my input).

E.g.
0,3,5,4,3,0

the instructions are:
while A > 0 :
    A = A / 8
    print A % 8

So at each step, the program right shifts A by 3 bits and prints the last 3 bits.

Do a BFS where each edge left shifts by 3 and adds a number between 0 and 7.
Can terminate paths where the output diverges from the program.

*/
use std::{collections::VecDeque, io};

#[allow(unused_imports)]
use utils::{pause, read_from_args};

#[derive(Debug, Clone, Copy)]
struct Registers {
    a: usize,
    b: usize, 
    c: usize
}

fn main() -> io::Result<()>{
    let input = read_from_args()?;
    let mut lines = input.lines();

    fn parse_register(line: &str) -> usize {
        line.chars().skip(12).collect::<String>().parse::<usize>().unwrap()
    }

    let mut registers = Registers {
        a: parse_register(lines.next().unwrap()),
        b: parse_register(lines.next().unwrap()),
        c: parse_register(lines.next().unwrap())
    };
    // dbg!(registers);
    lines.next();

    let program_string = &lines.next().unwrap()[9..]; 
    let program = program_string.split(",").map(|num| num.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    // dbg!(&program);

    let output_buffer = execute(&program, &mut registers);
    let output = output_buffer.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(",");
    dbg!(output);

    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(0);

    'bfs: while !queue.is_empty() {
        // println!("{:?}", &queue);
        let a = queue.pop_front().unwrap();

        // let output_buffer = execute_with(&program, a);
        // println!("{:?}", &output_buffer);
        // pause();
        for num in 0..8 {
            let new_a = (a << 3) + num;
            let output_buffer = execute_with(&program, new_a);
            // println!("{num} => {:?}", &output_buffer);
            if output_buffer != program[program.len() - output_buffer.len()..] {
                continue;
            }
            if output_buffer == program {
                dbg!(new_a);
                break 'bfs;
            }
            queue.push_back(new_a);
        }
    }
    Ok(())
}

fn execute_with(program: &Vec<usize>, a: usize) -> Vec<usize> {
    let mut registers = Registers {
        a: a,
        b: 0,
        c: 0
    };
    execute(program, &mut registers)
}

fn execute(program: &Vec<usize>, registers: &mut Registers) -> Vec<usize> {
    let mut instruction_pointer = 0;
    let mut output_buffer: Vec<usize> = Vec::new();

    loop {
        if instruction_pointer >= program.len() {
            break;
        }

        let (opcode, operand) = (program[instruction_pointer], program[instruction_pointer + 1]);
        instruction_pointer = match opcode {
            0 => adv(operand, registers, instruction_pointer, &mut output_buffer),
            1 => bxl(operand, registers, instruction_pointer, &mut output_buffer),
            2 => bst(operand, registers, instruction_pointer, &mut output_buffer),
            3 => jnz(operand, registers, instruction_pointer, &mut output_buffer),
            4 => bxc(operand, registers, instruction_pointer, &mut output_buffer),
            5 => out(operand, registers, instruction_pointer, &mut output_buffer),
            6 => bdv(operand, registers, instruction_pointer, &mut output_buffer),
            7 => cdv(operand, registers, instruction_pointer, &mut output_buffer),
            _ => unreachable!(),
        };
    }
    output_buffer
}

fn adv(operand: usize, registers: &mut Registers, instruction_pointer: usize, _output_buffer: &mut Vec<usize>) -> usize {
    // "division"
    let numerator = registers.a;
    let denominator = 2_usize.pow(get_combo_value(operand, registers) as u32);

    registers.a = numerator / denominator;
    instruction_pointer + 2
}

fn bxl(operand: usize, registers: &mut Registers, instruction_pointer: usize, _output_buffer: &mut Vec<usize>) -> usize {
    // bitwise XOR of B and operand
    registers.b = registers.b ^ operand;
    instruction_pointer + 2
}

fn bst(operand: usize, registers: &mut Registers, instruction_pointer: usize, _output_buffer: &mut Vec<usize>) -> usize {
    // modulo 8
    registers.b = get_combo_value(operand, registers) % 8;
    instruction_pointer + 2
}

fn jnz(operand: usize, registers: &mut Registers, instruction_pointer: usize, _output_buffer: &mut Vec<usize>) -> usize {
    // jump
    if registers.a == 0 {
        instruction_pointer + 2
    } else {
        operand
    }
}

fn bxc(_operand: usize, registers: &mut Registers, instruction_pointer: usize, _output_buffer: &mut Vec<usize>) -> usize {
    // bitwise XOR of B and C
    registers.b = registers.b ^ registers.c;
    instruction_pointer + 2
}

fn out(operand: usize, registers: &mut Registers, instruction_pointer: usize, output_buffer: &mut Vec<usize>) -> usize {
    // output mod 8
    output_buffer.push(
        get_combo_value(operand, registers) % 8
    );
    instruction_pointer + 2
}

fn bdv(operand: usize, registers: &mut Registers, instruction_pointer: usize, _output_buffer: &mut Vec<usize>) -> usize {
    // "division"
    let numerator = registers.a;
    let denominator = 2_usize.pow(get_combo_value(operand, registers) as u32);

    registers.b = numerator / denominator;
    instruction_pointer + 2
}

fn cdv(operand: usize, registers: &mut Registers, instruction_pointer: usize, _output_buffer: &mut Vec<usize>) -> usize {
    // "division"
    let numerator = registers.a;
    let denominator = 2_usize.pow(get_combo_value(operand, registers) as u32);

    registers.c = numerator / denominator;
    instruction_pointer + 2}

fn get_combo_value(operand: usize, registers: &Registers) -> usize {
    if operand <= 3 {
        operand
    } else if operand == 4 {
        registers.a
    } else if operand == 5 {
        registers.b
    } else if operand == 6 {
        registers.c
    } else {
        unreachable!()
    }
}
