use std::fs;

pub fn part_two() {
    let input = fs::read_to_string("input").expect("Something went wrong reading the file");

    let sequences = input
        .lines()
        .map(|x| {
            x.split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut predictions_sum = 0;

    for sequence in sequences {
        let mut differences = vec![sequence];

        loop {
            let mut curr_diffs = Vec::new();

            for j in 0..differences.last().unwrap().len() - 1 {
                curr_diffs.push(differences.last().unwrap()[j + 1] - differences.last().unwrap()[j])
            }

            if curr_diffs.iter().sum::<i32>() == 0 {
                break;
            }
            differences.push(curr_diffs);
        }

        let mut prediction = 0;
        for diffs in differences.iter().rev() {
            prediction = diffs.first().unwrap() - prediction;
        }

        predictions_sum += prediction;
    }

    println!("Sum of predictions: {}", predictions_sum);
}
