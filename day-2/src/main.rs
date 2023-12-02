use std::fs;

fn part_1() {
    let input = fs::read_to_string("input").expect("Error reading input.");

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut ids_sum: u32 = 0;

    for line in input.lines() {
        let line_split = line.split(':').collect::<Vec<&str>>();
        let game_data = line_split[0];
        let game_sets = line_split[1].split_whitespace().collect::<String>();

        let game_id: u32 = game_data[5..].parse().unwrap();
        let mut selected = true;

        for set in game_sets.split(";") {
            let mut reds: u8 = 0;
            let mut greens: u8 = 0;
            let mut blues: u8 = 0;

            for draw in set.split(",") {
                if draw.contains("red") {
                    reds = draw[..draw.len() - 3].parse().unwrap();
                } else if draw.contains("green") {
                    greens = draw[..draw.len() - 5].parse().unwrap();
                } else if draw.contains("blue") {
                    blues = draw[..draw.len() - 4].parse().unwrap();
                }
            }

            if reds > max_red || greens > max_green || blues > max_blue {
                selected = false;
                break;
            }
        }

        if selected {
            ids_sum += game_id;
        }
    }

    println!("Sum of IDs: {}", ids_sum);
}

fn part_2() {
    let input = fs::read_to_string("input").expect("Error reading input.");
    let mut powers_sum: u32 = 0;

    for line in input.lines() {
        let line_split = line.split(':').collect::<Vec<&str>>();
        let game_sets = line_split[1].split_whitespace().collect::<String>();

        let mut max_reds: u32 = 0;
        let mut max_greens: u32 = 0;
        let mut max_blues: u32 = 0;

        for set in game_sets.split(";") {
            let mut reds: u32 = 0;
            let mut greens: u32 = 0;
            let mut blues: u32 = 0;

            for draw in set.split(",") {
                if draw.contains("red") {
                    reds = draw[..draw.len() - 3].parse().unwrap();
                } else if draw.contains("green") {
                    greens = draw[..draw.len() - 5].parse().unwrap();
                } else if draw.contains("blue") {
                    blues = draw[..draw.len() - 4].parse().unwrap();
                }
            }

            if reds > max_reds {
                max_reds = reds;
            }
            if greens > max_greens {
                max_greens = greens;
            }
            if blues > max_blues {
                max_blues = blues;
            }
        }

        powers_sum += max_reds * max_greens * max_blues;
    }

    println!("Sum of powers: {}", powers_sum);
}

fn main() {
    part_1();
    part_2();
}
