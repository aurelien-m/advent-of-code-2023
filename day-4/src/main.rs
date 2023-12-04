use std::fs;

fn part_one() {
    let input = fs::read_to_string("input").expect("Error reading input file");
    let mut score_sum = 0;

    for line in input.lines() {
        let line_split = line.split(':').collect::<Vec<&str>>();
        let game = line_split[1].split('|').collect::<Vec<&str>>();

        let winning_numbers = game[0]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let game_numbers = game[1]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let mut score = 0;

        for number in game_numbers {
            if winning_numbers.contains(&number) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }

        score_sum += score;
    }

    println!("Score sum: {}", score_sum);
}

fn part_two() {
    let input = fs::read_to_string("input").expect("Error reading input file");

    let mut cards = vec![1; input.lines().count()];

    for line in input.lines() {
        let line_split = line.split(':').collect::<Vec<&str>>();
        let game_idx = line_split[0][4..]
            .split_whitespace()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let game = line_split[1].split('|').collect::<Vec<&str>>();

        let winning_numbers = game[0]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let game_numbers = game[1]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        for _ in 0..cards[game_idx - 1] {
            let mut wins = 0;
            for number in &game_numbers {
                if winning_numbers.contains(&number) {
                    cards[game_idx + wins] += 1;
                    wins += 1;
                }
            }
        }
    }

    println!("Sum of cards: {}", cards.iter().sum::<usize>());
}

fn main() {
    part_one();
    part_two();
}
