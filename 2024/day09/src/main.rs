/*
Part 1: Given a "disk map" like 12345
it represents a sequence of free space / file blocks:
0..111....22222
where each file is represented by an ID which is the
order it appears.
Move file blocks from the end to the leftmost free space
until there are no gaps:
02211122......
What's the resulting checksum?
checksum = sum_i position_i id_i

Approach: map each block to a tuple (id, key)
where key is 0 for free space and 1 for file blocks.
For a n*logn running time, scan from left and right,
swap if left is empty and right has a block

Part 2: Restrict to moving whole files.

Approach: same as above, but only swap if the whole file
on the right fits in the empty space on the left
*/
use std::io;

use utils::read_from_args;

fn main() -> io::Result<()> {
    let disk_map = read_from_args()?;

    // O(n)
    #[allow(unused_mut)]
    let mut file_blocks: Vec<isize> = disk_map.chars().enumerate().map(
        |(i, number)| {
            let length = number.to_digit(10).unwrap();
            if i % 2 == 0 {
                // file block
                vec![i.div_ceil(2) as isize; length as usize]
            }  else {
                // empty space
                vec![-1; length as usize]
            }
        }
    ).flatten().collect();
    // dbg!(&file_blocks);

    // selection "compact" O(n^2)
    // selection_compact(&mut file_blocks);
    // quick "compact" O(nlogn)
    // quick_compact(&mut file_blocks);
    // contiguous quick compact O(nlogn)
    contiguous_quick_compact(&mut file_blocks);
    // dbg!(&file_blocks);

    // O(n)
    let checksum: usize = file_blocks.iter().enumerate().fold(0, 
        |checksum, (position, id)| if *id >= 0 {checksum + position * (*id) as usize} else {checksum}
    );
    dbg!(checksum);
    Ok(())
}

#[allow(dead_code)]
fn selection_compact(sequence: &mut Vec<isize>) {
    for i in 0..sequence.len() {
        for j in (i..sequence.len()).rev() {
            if sequence[i] >= 0 {continue};

            if sequence[i] < sequence[j]{
                sequence.swap(i, j);
            }
        }
    }
}

#[allow(dead_code)]
fn quick_compact(sequence: &mut Vec<isize>) {
    let size = sequence.len();

    let (mut left, mut right) = (0, size - 1);
    while left < right {
        // print_sequence(&sequence, left, right);
        if sequence[left] < 0 {
            if sequence[right] >= 0 {
                sequence.swap(left, right);
                left += 1;
            }
            right = right.saturating_sub(1);
        } else {
            if sequence[right] >= 0 {
                left += 1;
            } else {
                right = right.saturating_sub(1);
            }
        }
    }
}

fn contiguous_quick_compact(sequence: &mut Vec<isize>) {
    let size = sequence.len();

    let (mut left, mut right) = (0, size - 1);
    find_next_file_right(sequence, &mut right);

    while right > 0 {
        find_next_empty_left(sequence, &mut left);

        if left >= right {
            // println!("no empty space");
            // reset left
            left = 0;
            // find next file
            right -= 1;
            find_next_file_right(sequence, &mut right);
        }

        // print_sequence(&sequence, left, right);

        if is_empty(&sequence, left) &&
        is_file(&sequence, right) &&
        block_size(sequence, left) >=
        block_size(sequence, right) {
            swap_blocks(sequence, left, right);
            // println!("swap {left} {right}");
            // reset left
            left = 0;
            // find next file
            find_next_file_right(sequence, &mut right);
        } 


    }

    // loop {
    //     let success = find_next_block_right(sequence, &mut right);
    //     if !success {return;}
    //     find_next_empty_left(sequence, &mut left);
    //     print_sequence(&sequence, left, right);

    //     if left >= right {
    //         right -= 1;
    //         left = 0;
    //         continue;
    //     }

    //     if block_size(sequence, left) >=
    //     block_size(sequence, right) {
    //         // println!("swap {} {}", left, right);
    //         swap_block(sequence, &mut left, &mut right);
    //         left = 0;
    //     }
    // }
}

fn find_next_file_right(sequence: &Vec<isize>, position: &mut usize){
    while is_empty(sequence, *position)
    || sequence[*position] == sequence[(*position).saturating_sub(1)] {
        if *position == 0 {
            break;
        }
        *position -= 1;
    };
}

fn find_next_empty_left(sequence: &Vec<isize>, position: &mut usize) {
    while is_empty(sequence, *position)
    || sequence[*position] == sequence[(*position)+1] {
        *position += 1;
        if *position + 1 == sequence.len() {
            break
        }
    }
    *position += 1
}

fn block_size(sequence: &mut Vec<isize>, position: usize) -> usize {
    let mut size = 1;
    let mut temp_position = position;
    while temp_position + 1 < sequence.len() 
        && sequence[temp_position] == sequence[temp_position+1] {
        size += 1;
        temp_position += 1;
    };
    size
}

fn is_empty(sequence: &Vec<isize>, position: usize) -> bool {
    sequence[position] < 0
}

fn is_file(sequence: &Vec<isize>, position: usize) -> bool {
    sequence[position] >= 0
}

fn swap_blocks(sequence: &mut Vec<isize>, left: usize, right: usize) {
    if is_file(sequence, left) || is_empty(sequence, right) {
        panic!("Unexpected: attempted wrong swap.")
    }
    let (mut temp_left, mut temp_right) = (left, right);
    sequence.swap(temp_left, temp_right);
    (temp_left, temp_right) = (temp_left + 1, temp_right + 1);

    while temp_right < sequence.len() && sequence[temp_right] == sequence[temp_left-1] {
        sequence.swap(temp_left, temp_right);
        (temp_left, temp_right) = (temp_left + 1, temp_right + 1);
    }
}

#[allow(dead_code)]
fn print_sequence(sequence: &Vec<isize>, left: usize, right: usize) {
    println!("{}", &sequence.iter().map(
        |v| if *v >= 0 {
            v.to_string()
        } else {'.'.to_string()}
    ).collect::<String>()
    );
    println!(
        "{symbol:>left$}{symbol:>right$}",
        symbol="^",
        left=left+1,
        right=right.saturating_sub(left)
    );
}