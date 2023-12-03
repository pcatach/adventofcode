use std::fs;
use std::io;

const GEAR_SYMBOL: char = '*';
const EMPTY_SYMBOL: char = '.';

type Number = (i32, usize, usize);
type Symbol = (char, usize, usize);

fn main() -> io::Result<()> {
    let file_path = "input.txt";
    let scheme = fs::read_to_string(file_path)?;
    compute_gear_ratios(&scheme);
    Ok(())
}

fn compute_gear_ratios(scheme: &str) {
    let (symbols, numbers) = process_scheme(&scheme);
    // println!("Symbols: {:?}", symbols);
    // println!("Numbers: {:?}", numbers);

    let (part_numbers, sum_part_numbers) = get_part_numbers(&symbols, &numbers);
    println!("Sum of part numbers: {}", sum_part_numbers);

    let mut sum_of_gear_ratios = 0;
    for symbol in symbols {
        if symbol.0 == GEAR_SYMBOL {
            // println!("Potential gear found: {:?}", symbol);
            let mut gear_neighbors: Vec<Number> = Vec::new();
            for number in &part_numbers {
                if check_is_neighbor(&number, &symbol) {
                    gear_neighbors.push(*number);
                }
            }
            if gear_neighbors.len() == 2 {
                let mut gear_ratio: i32 = 1;
                for neighbor in gear_neighbors {
                    gear_ratio *= neighbor.0;
                }
                sum_of_gear_ratios += gear_ratio;
            }
        }
    }
    println!("Sum of gear ratios: {}", sum_of_gear_ratios)
}

fn process_scheme(scheme: &str) -> (Vec<Symbol>, Vec<Number>) {
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();

    for (i, line) in scheme.lines().enumerate() {
        let mut j = 0;
        while j < line.len() {
            let c = line.chars().nth(j).unwrap();
            if c == EMPTY_SYMBOL {
                j += 1;
            } else if c.is_numeric() {
                let number_str: &str = get_number_at(line, j);
                let number = number_str.parse::<i32>().unwrap();
                numbers.push((number, i, j));
                j += number_str.len();
            } else {
                symbols.push((c, i, j));
                j += 1;
            }
        }
    }
    return (symbols, numbers);
}

fn get_part_numbers(symbols: &Vec<Symbol>, numbers: &Vec<Number>) -> (Vec<Number>, i32) {
    let mut part_numbers: Vec<Number> = Vec::new();
    let mut sum_part_numbers = 0;

    for number in numbers {
        if check_is_part_number(number, symbols) {
            part_numbers.push(*number);
            sum_part_numbers += number.0;
        }
    }
    return (part_numbers, sum_part_numbers);
}

fn check_is_part_number(number: &Number, symbols: &Vec<Symbol>) -> bool {
    let mut is_part_number = false;
    for symbol in symbols {
        if check_is_neighbor(number, symbol) {
            is_part_number = true;
            // println!("{} is neighbor of {}", number.0, symbol.0);
        }
    }
    return is_part_number;
}

fn check_is_neighbor(number: &Number, symbol: &Symbol) -> bool {
    let diff_rows = number.1 as i32 - symbol.1 as i32;
    if diff_rows.abs() > 1 {
        return false;
    }

    for i in 0..number.0.to_string().len() {
        let diff_cols = number.2 as i32 + i as i32 - symbol.2 as i32;

        if diff_cols.abs() <= 1 {
            return true;
        }
    }
    return false;
}

fn get_number_at(line: &str, position: usize) -> &str {
    let mut i: usize = position;
    while line.chars().nth(i).unwrap().is_numeric() {
        i += 1;
        if i == line.len() {
            break;
        }
    }
    return &line[position..i];
}
