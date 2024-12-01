pub fn solution(seeds: Vec<u128>, maps: &part1::Maps) {
    for location in 0..inf {
        seed = apply_maps_backwards(location, maps);
        if seed_exists(seed, seeds) {
            println!("Lowest location number: {}", location);
            return;
        }
    }
}

fn apply_maps_backwards(location: u128, maps: &part1::Maps) {}
