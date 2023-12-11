use std::fs;

fn main() {
    let file_path = "input.txt";
    let input = fs::read_to_string(file_path).expect("Could not read file");

    let sequences = parse_sequences(&input);
    // println!("{:?}", sequences);

    let extrapolated_sequences: Vec<Vec<i64>> = sequences
        .iter()
        .map(|sequence: &Vec<i64>| extrapolate_backwards(sequence))
        .collect();

    // println!("Extrapolated sequences: \n{:?}", extrapolated_sequences);
    // let sum_of_next_values: i64 = extrapolated_sequences
    //     .iter()
    //     .map(|sequence| sequence.last().unwrap())
    //     .sum();
    let sum_of_previous_values: i64 = extrapolated_sequences
        .iter()
        .map(|sequence| sequence.first().unwrap())
        .sum();
    println!("Sum of previous values: {sum_of_previous_values}")
}

fn extrapolate_backwards(sequence: &Vec<i64>) -> Vec<i64> {
    let diff_sequence: Vec<i64> = sequence.windows(2).map(|x| x[1] - x[0]).collect();
    let mut new_sequence = sequence.clone();

    if diff_sequence.iter().all(|x| *x == 0) {
        return new_sequence;
    }
    new_sequence.insert(
        0,
        *sequence.first().unwrap() - extrapolate_backwards(&diff_sequence).first().unwrap(),
    );
    new_sequence
}

fn extrapolate(sequence: &Vec<i64>) -> Vec<i64> {
    let diff_sequence: Vec<i64> = sequence.windows(2).map(|x| x[1] - x[0]).collect();
    let mut new_sequence = sequence.clone();

    if diff_sequence.iter().all(|x| *x == 0) {
        return new_sequence;
    }
    new_sequence.push(*sequence.last().unwrap() + extrapolate(&diff_sequence).last().unwrap());
    new_sequence
}

fn parse_sequences(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i64>().expect("Could not parse integer"))
                .collect()
        })
        .collect()
}
