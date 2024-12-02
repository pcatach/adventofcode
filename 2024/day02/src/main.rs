use std::io;

use utils::read_from_args;

fn main() -> io::Result<()> {
    let report = read_from_args()?;
    println!("{report}");

    let safe_reports: u128 = report.lines().map(
        |levels_str| apply_is_safe(levels_str) as u128
    ).sum();
    println!("{safe_reports}");
    Ok(())
}

fn apply_is_safe(levels_str: &str) -> bool {
    let levels: Vec<i128> = levels_str.split_whitespace().map(
        |level| level.parse::<i128>().unwrap()
    ).collect();
    if is_safe(&levels) {
        return true;
    }

    for (index, _) in levels.iter().enumerate() {
        let mut l = levels.clone();
        l.remove(index);
        if is_safe(&l) {
            return true;
        }
    };
    false
}

fn is_safe(levels: &Vec<i128>) -> bool {
    let diffs = levels.windows(2).map(|w| w[1] - w[0]);
    
    let increasing_within_bounds = diffs.clone().all(|diff| diff > 0 && diff <= 3);
    let decreasing_within_bounds = diffs.clone().all(|diff| diff < 0 && diff >= -3);
    increasing_within_bounds || decreasing_within_bounds
}