/* Part 1: Given a word search problem, find all instances of "XMAS".
It can be horizontal, vertical, diagonal backwards, and overlapping.
 */
use std::io;
use std::cmp::min;

use utils::{read_from_args, read_array_from_string};

fn main() -> io::Result<()> {
    let word_search = read_from_args()?;
    let word_search_array = read_array_from_string(word_search);

    let horizontal_matches = get_horizontal_matches(&word_search_array);
    dbg!(horizontal_matches);
    let vertical_matches = get_vertical_matches(&word_search_array);
    dbg!(vertical_matches);
    let diagonal_1_matches = get_diagonal_1_matches(&word_search_array);
    dbg!(diagonal_1_matches);
    let diagonal_2_matches = get_diagonal_2_matches(&word_search_array);
    dbg!(diagonal_2_matches);
    dbg!(horizontal_matches + vertical_matches + diagonal_1_matches + diagonal_2_matches);

    let x_mas_matches = get_x_mas_matches(&word_search_array); // facepalm
    dbg!(x_mas_matches);

    Ok(())
}

fn get_x_mas_matches(word_search_array: &Vec<Vec<char>>) -> usize {
    let num_lines = word_search_array.len();
    let num_cols = word_search_array.first().unwrap().len();
    
    let mut count = 0;
    for i in 0..(num_lines-2) {
        for j in 0..(num_cols-2) {
            let sub_array = word_search_array
                .iter().skip(i).take(3)
                .map(|line| 
                    line.iter().skip(j).take(3).collect::<Vec<_>>()
                )
                .collect::<Vec<Vec<_>>>();
            let diagonal1 = [(0,0), (1,1), (2,2)].iter().map(
                |(i,j)| sub_array[*i][*j]
            ).collect::<String>();
            let diagonal2 = [(0,2), (1,1), (2,0)].iter().map(
                |(i,j)| sub_array[*i][*j]
            ).collect::<String>();
            if (diagonal1 == "MAS"
                || diagonal1 == "SAM")
                && ( diagonal2 == "MAS"
                || diagonal2 == "SAM") {
                count += 1
            }
        }
    }
    count
}

fn get_horizontal_matches(word_search_array: &Vec<Vec<char>>) -> usize {
    let num_lines = word_search_array.len();
    let num_cols = word_search_array.first().unwrap().len();

    let mut count = 0;
    for i in 0..num_lines  {
        let mut line: Vec<char> = Vec::new();
        for j in 0..num_cols {
            line.push(word_search_array[i][j]);
        }
        count += find_xmas_count(line);
    }
    count
}

fn get_vertical_matches(word_search_array: &Vec<Vec<char>>) -> usize {
    let num_lines = word_search_array.len();
    let num_cols = word_search_array.first().unwrap().len();

    let mut count = 0;
    for j in 0..num_cols  {
        let mut line: Vec<char> = Vec::new();
        for i in 0..num_lines {
            line.push(word_search_array[i][j]);
        }
        count += find_xmas_count(line);
    }
    count
}

fn get_diagonal_1_matches(word_search_array: &Vec<Vec<char>>) -> usize {
    let num_lines = word_search_array.len();
    let num_cols = word_search_array.first().unwrap().len();

    let mut count = 0;
    for d in 0..(num_lines+num_cols-1)  {
        let mut line: Vec<char> = Vec::new();
        let min_i = (d + 1).checked_sub(num_cols).unwrap_or(0); // max(0, d - num_cols + 1)
        let max_i = min(num_lines, d + 1);
        // dbg!(d, min_i, max_i);
        for i in min_i..max_i {
            // dbg!(i, d-i);
            line.push(word_search_array[i][d - i]);
        }
        count += find_xmas_count(line);
    }
    count
}

fn get_diagonal_2_matches(word_search_array: &Vec<Vec<char>>) -> usize {
    let num_lines = word_search_array.len();
    let num_cols = word_search_array.first().unwrap().len();

    let mut count = 0;
    for d in 0..(num_lines+num_cols-1)  {
        let mut line: Vec<char> = Vec::new();
        let min_i = (d + 1).checked_sub(num_cols).unwrap_or(0); // max(0, d - num_cols + 1)
        let max_i = min(num_lines, d + 1);
        // dbg!(d, min_i, max_i);
        for i in min_i..max_i {
            // dbg!(i, num_cols + i - d - 1);
            line.push(word_search_array[i][num_cols + i - d - 1]);
        }
        count += find_xmas_count(line);
    }
    count
}

fn find_xmas_count(line: Vec<char>) -> usize {
    line.windows(4).filter(|w| {
        let pattern = w.iter().collect::<String>();
        pattern == "XMAS" || pattern == "SAMX"
    }).count()
}
