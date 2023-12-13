use std::cmp::{max, min};
use std::fs;

fn main() {
    let expansion_factor = 1_000_000;
    let file_path = "input.txt";
    let image_input = fs::read_to_string(file_path).expect("Could not read file");
    let image: Vec<Vec<char>> = image_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // expand universe
    // print_image(&image);
    let (expanded_image, rows_to_expand, cols_to_expand) = expand(&image);
    // println!();
    // print_image(&image);

    // find galaxies
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    // for (i, row) in expanded_image.iter().enumerate() {
    for (i, row) in image.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == '#' {
                galaxies.push((i, j));
            }
        }
    }

    // compute pairwise distances
    let mut sum_of_shortest_lengths = 0;
    while !galaxies.is_empty() {
        let (first_galaxy, rest) = galaxies.split_first().unwrap();
        for galaxy in rest.iter() {
            let num_rows_to_expand = (expansion_factor - 1)
                * rows_to_expand
                    .iter()
                    .filter(|&&r| {
                        min(first_galaxy.0, galaxy.0) <= r && r <= max(first_galaxy.0, galaxy.0)
                    })
                    .count();
            let num_cols_to_expand = (expansion_factor - 1)
                * cols_to_expand
                    .iter()
                    .filter(|&&c| {
                        min(first_galaxy.1, galaxy.1) <= c && c <= max(first_galaxy.1, galaxy.1)
                    })
                    .count();

            sum_of_shortest_lengths += (first_galaxy.0 as isize - galaxy.0 as isize).abs()
                + (first_galaxy.1 as isize - galaxy.1 as isize).abs()
                + num_rows_to_expand as isize
                + num_cols_to_expand as isize;
        }
        galaxies = rest.to_vec();
    }
    println!("The sum of the shortest lengths between all pairs of galaxies is {sum_of_shortest_lengths}")
}

fn expand(image: &Vec<Vec<char>>) -> (Vec<Vec<char>>, Vec<usize>, Vec<usize>) {
    let (expanded_image, rows_to_expand) = expand_rows(image);
    let expanded_image_t = transpose(&expanded_image);
    let (expanded_image_t, cols_to_expand) = expand_rows(&expanded_image_t);
    (transpose(&expanded_image_t), rows_to_expand, cols_to_expand)
}

fn expand_rows(image: &Vec<Vec<char>>) -> (Vec<Vec<char>>, Vec<usize>) {
    let mut rows_to_expand: Vec<usize> = Vec::new();
    let mut expanded_image: Vec<Vec<char>> = Vec::new();
    for (i, row) in image.iter().enumerate() {
        expanded_image.push(row.clone());

        if !row.contains(&'#') {
            expanded_image.push(row.clone());
            rows_to_expand.push(i)
        }
    }
    (expanded_image, rows_to_expand)
}

fn transpose(image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = image.len();
    let cols = image.first().map_or(0, |row| row.len());

    let mut transposed_image: Vec<Vec<char>> = vec![vec!['.'; rows]; cols];
    for (i, row) in image.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            transposed_image[j][i] = *val;
        }
    }
    transposed_image
}

fn print_image(image: &Vec<Vec<char>>) {
    for row in image.iter() {
        let line: String = row.iter().collect();
        println!("{line}");
    }
}
