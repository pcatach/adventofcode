use std::fs;
use std::io;

pub type Map = Vec<(u128, u128, u128)>;
pub type Maps = Vec<Map>;

fn main() -> io::Result<()> {
    let file_path = "input.txt";
    let almanac_input = fs::read_to_string(file_path)?;

    let (seeds, maps) = parse_almanac(&almanac_input);
    solution2(seeds, &maps);

    Ok(())
}

fn parse_almanac(input: &str) -> (Vec<u128>, Maps) {
    let mut seeds: Vec<u128> = Vec::new();
    let mut maps: Maps = Vec::new();
    for section in input.split("\n\n") {
        let (prefix, rest) = section
            .split_once(":")
            .expect("Could not find section header");

        if prefix == "seeds" {
            seeds = rest
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<u128>().unwrap())
                .collect();
        } else {
            let mut map: Map = Vec::new();
            for line in rest.trim().lines() {
                let map_entry: Vec<u128> = line
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<u128>().unwrap())
                    .collect();
                map.push((map_entry[0], map_entry[1], map_entry[2]));
            }
            maps.push(map);
        }
    }
    return (seeds, maps);
}

pub fn solution1(seeds: Vec<u128>, maps: &Maps) {
    let locations: Vec<u128> = seeds
        .iter()
        .map(|&source| apply_maps(source, 0, &maps))
        .collect();

    let lowest_location_number = locations.iter().min().unwrap();
    println!("Lowest location number: {}", lowest_location_number);
}

fn apply_maps(source: u128, map_index: usize, maps: &Maps) -> u128 {
    let mut destination = source;
    for entry in maps[map_index].iter() {
        let (destination_range_start, source_range_start, range_length) = entry;

        if (*source_range_start..source_range_start + range_length).contains(&source) {
            destination = destination_range_start + (source - source_range_start);
        }
    }
    // println!("source: {}, dest: {}", source, destination);
    if map_index + 1 == maps.len() {
        return destination;
    } else {
        return apply_maps(destination, map_index + 1, maps);
    }
}

pub fn solution2(seeds: Vec<u128>, maps: &Maps) {
    let mut location: u128 = 0;
    loop {
        let seed: u128 = apply_maps_backwards(location, maps.len() - 1, maps);
        if seed_exists(seed, &seeds) {
            println!("Lowest location number: {}", location);
            break;
        };
        location += 1;
    }
}

fn apply_maps_backwards(destination: u128, map_index: usize, maps: &Maps) -> u128 {
    let mut source = destination;
    for entry in maps[map_index].iter() {
        let (destination_range_start, source_range_start, range_length) = entry;

        if (*destination_range_start..destination_range_start + range_length).contains(&destination)
        {
            source = source_range_start + (destination - destination_range_start);
        }
    }
    // println!(
    //     "source: {}, dest: {}, map: {:?}",
    //     source, destination, maps[map_index],
    // );
    if map_index == 0 {
        return source;
    } else {
        return apply_maps_backwards(source, map_index - 1, maps);
    }
}

fn seed_exists(seed: u128, seeds: &Vec<u128>) -> bool {
    let mut i: usize = 0;
    while i < seeds.len() {
        if (seeds[i]..seeds[i] + seeds[i + 1]).contains(&seed) {
            return true;
        }
        i += 2;
    }
    return false;
}
