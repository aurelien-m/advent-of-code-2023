use std::cmp;
use std::fs;

fn part_one() {
    let input = fs::read_to_string("input").expect("Error reading input");

    let seeds = &input.lines().next().unwrap()[7..]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let parsed_input = &input
        .split("\n\n")
        .map(|s| s.split("\n").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()[1..];

    let mut mappings = Vec::new();

    for map_str in parsed_input {
        let mut map = Vec::new();

        for set_str in &map_str[1..] {
            let set = set_str
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            map.push(set);
        }

        mappings.push(map);
    }

    let mut first_to_last = Vec::new();

    for seed in seeds {
        let mut curr_id = *seed;

        for mapping in &mappings {
            for set in mapping {
                let dest_start = set[0];
                let source_start = set[1];
                let step = set[2];

                if &curr_id >= &source_start && &curr_id < &(source_start + step) {
                    curr_id = &curr_id - source_start + dest_start;
                    break;
                }
            }
        }

        first_to_last.push((seed, curr_id.clone()));
    }

    let min = first_to_last.iter().min_by_key(|(_, id)| id).unwrap();
    println!("Min: {:?} (seed, location)", min);
}

fn intersection(a_min: u64, a_max: u64, b_min: u64, b_max: u64) -> Option<(u64, u64)> {
    if b_min > a_max || a_min > b_max {
        return None;
    }

    return Some((cmp::max(a_min, b_min), cmp::min(a_max, b_max)));
}

fn difference(a_min: u64, a_max: u64, b_min: u64, b_max: u64) -> Option<Vec<u64>> {
    // Can be [min, max] or [min, max, min, max] in the case of two differences

    if b_min > a_max || a_min > b_max {
        return None;
    }

    let mut differences = Vec::new();
    let intersection = intersection(a_min, a_max, b_min, b_max).unwrap();

    if a_min < intersection.0 {
        differences.push(a_min);
        differences.push(intersection.0 - 1);
    }

    if a_max > intersection.1 {
        differences.push(intersection.1 + 1);
        differences.push(a_max);
    }

    return Some(differences);
}

fn part_two() {
    let input = fs::read_to_string("input").expect("Error reading input");

    let seeds_ranges = &input.lines().next().unwrap()[7..]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let parsed_input = &input
        .split("\n\n")
        .map(|s| s.split("\n").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()[1..];

    let mut mappings = Vec::new();

    for map_str in parsed_input {
        let mut map = Vec::new();

        for set_str in &map_str[1..] {
            let set = set_str
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            map.push(set);
        }

        mappings.push(map);
    }

    let mut seeds = Vec::new();
    for range in seeds_ranges.chunks(2) {
        seeds.push(range[0]);
        seeds.push(range[0] + range[1] - 1);
    }

    println!("Seeds: {:?}", seeds);

    let mut all_ids = Vec::new();
    all_ids.push(seeds);

    for level in 0..mappings.len() {
        println!("Level: {}", level);
        let mut next_ids = Vec::new();

        for seed in all_ids[level].clone().chunks(2) {
            let start = seed[0];
            let end = seed[1];
            let mut intersections = Vec::new();

            println!("Start: {}, End: {}", start, end);

            for set in mappings[level].clone() {
                let dest_start = set[0];
                let source_start = set[1];
                let step = set[2];

                println!(
                    "source start: {}, source end: {}",
                    source_start,
                    source_start + step
                );

                if let Some(intersection) =
                    intersection(start, end, source_start, source_start + step)
                {
                    println!("Intersection: {:?}", intersection);
                    next_ids.push(&intersection.0 - source_start + dest_start);
                    next_ids.push(&intersection.1 - source_start + dest_start);
                    intersections.push(intersection);
                }
            }

            if intersections.len() == 0 {
                next_ids.push(start);
                next_ids.push(end);
            } else {
                let mut differences = vec![start, end];

                for intersection in &intersections {
                    let mut new_differences = Vec::new();

                    for diff in differences.chunks(2) {
                        let s = diff[0];
                        let e = diff[1];

                        if let Some(d) = difference(s, e, intersection.0, intersection.1) {
                            println!("Difference: {:?}", &d);
                            for range in d.chunks(2) {
                                new_differences.push(range[0]);
                                new_differences.push(range[1]);
                            }
                        }
                    }

                    differences = new_differences;
                }

                for diff in differences.chunks(2) {
                    next_ids.push(diff[0]);
                    next_ids.push(diff[1]);
                }
            }

            println!("Next IDs: {:?}", next_ids);
        }

        all_ids.push(next_ids);
    }

    println!("All IDs: {:?}", all_ids);
    println!(
        "Min last level: {}",
        all_ids.last().unwrap().iter().min().unwrap()
    );
}

fn main() {
    part_one();
    part_two();
}
