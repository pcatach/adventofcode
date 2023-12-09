use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let file_path = "input.txt";
    let races_input = fs::read_to_string(file_path)?;

    let (times, distances) = parse_races(&races_input);
    println!("{:?}\n{:?}", times, distances);

    // 1. Brute force
    // let mut numbers: Vec<u128> = Vec::new();
    // for (i, &time) in times.iter().enumerate() {
    //     let mut ways_to_beat_record = 0;
    //     for button_time in 0..time {
    //         if beats_record(button_time, distances[i], time) {
    //             ways_to_beat_record += 1;
    //         }
    //     }
    //     numbers.push(ways_to_beat_record);
    // }

    // 2. Maths
    let numbers = times
        .iter()
        .enumerate()
        .map(|(i, &time)| compute_number_of_ways(time, distances[i]));

    println!("{:?}", numbers.product::<u128>());
    Ok(())
}

fn parse_races(races_input: &str) -> (Vec<u128>, Vec<u128>) {
    let mut lines = races_input.lines();

    let time_line = lines.next().expect("No times found");
    let distance_line = lines.next().expect("No distances found");

    return (
        parse_line(time_line, "Time: "),
        parse_line(distance_line, "Distance: "),
    );
}

fn parse_line(line: &str, prefix: &str) -> Vec<u128> {
    vec![line
        .strip_prefix(prefix)
        .unwrap_or_else(|| panic!("Expected prefix {}", prefix))
        .trim()
        .replace(" ", "")
        .parse::<u128>()
        .unwrap()]
}

fn parse_line_old(line: &str, prefix: &str) -> Vec<u128> {
    line.strip_prefix(prefix)
        .unwrap_or_else(|| panic!("Expected prefix {}", prefix))
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect::<Vec<u128>>()
}

fn beats_record(button_time: u128, distance: u128, record: u128) -> bool {
    if button_time == 0 {
        return false;
    }
    let remaining_time = distance / button_time;
    return button_time + remaining_time < record;
}

fn compute_number_of_ways(time: u128, record: u128) -> u128 {
    // Let's say distance is D, the time is T,
    // and the time for holding the button is t.
    // The velocity after the button is released
    // is given by v = t, so the total distance is
    // D = v * (T-t) = t * (T - t)
    //
    // We need to find all t such that D <= record:
    // t * (T - t) >= record
    // - t^2 + T*t - record >= 0
    // if the zeros of the polynomial on the LHS are t_0 and t_1, then
    // (t - t_0) * (t - t_1) >= 0
    // => min(t_0,t_1) <= t <= max(t_0,t_1)
    // so the number of ways to beat the record is
    // floor(t_1) - ceil(t_0) + 1 (the +1 is for the first integer in the range)

    let delta = time.pow(2) - 4 * record;
    let t_0 = (time as f64 - (delta as f64).sqrt()) / 2 as f64;
    let t_1 = (time as f64 + (delta as f64).sqrt()) / 2 as f64;

    return (t_1.floor() - t_0.ceil()) as u128 + 1;
}
