use std::fs;

pub fn part_one() {
    let input = fs::read_to_string("input").expect("Something went wrong reading the file");

    let split = input
        .lines()
        .map(|line| {
            line[11..]
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let times = &split[0];
    let distances = &split[1];
    let mut possible_wins_mul = 1;

    for race in 0..times.len() {
        let time = times[race];
        let record_distance = distances[race];
        let mut possibles_wins_count = 0;

        for hold_time in 1..time {
            let speed = hold_time;
            let distance = speed * (time - hold_time);

            if distance > record_distance {
                possibles_wins_count += 1;
            }
        }

        possible_wins_mul *= possibles_wins_count;
    }

    println!("Part one answer: {}", possible_wins_mul);
}
