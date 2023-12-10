use lcmx::lcmx;
use std::collections::HashMap;
use std::fs;

pub fn part_two_slow() {
    let input = fs::read_to_string("input").expect("Something went wrong reading the file");

    let mut split = input.split("\n\n");
    let instructions = split.next().unwrap().chars().collect::<Vec<char>>();
    let network_list = split
        .next()
        .unwrap()
        .split("\n")
        .map(|x| vec![&x[0..3], &x[7..10], &x[12..15]])
        .collect::<Vec<Vec<&str>>>();

    let network = network_list
        .iter()
        .map(|x| (x[0].to_string(), vec![x[1].to_string(), x[2].to_string()]))
        .collect::<HashMap<String, Vec<String>>>();

    let mut instruction_index = 0;
    let mut current_nodes = Vec::new();
    for key in network.keys() {
        if key.ends_with('A') {
            current_nodes.push(key.to_string());
        }
    }

    let mut step: u64 = 0;
    let mut finished;

    loop {
        current_nodes = current_nodes
            .iter()
            .map(|x| match instructions[instruction_index] {
                'L' => network[x][0].clone(),
                'R' => network[x][1].clone(),
                _ => panic!("Unknown instruction"),
            })
            .collect::<Vec<String>>();

        instruction_index += 1;
        if instruction_index == instructions.len() {
            instruction_index = 0;
        }

        step += 1;
        if step % 1_000_000 == 0 {
            println!(
                "Step: {} ({} %)",
                step,
                step as f64 / 8_811_050_362_409.0 * 100.0
            );
        }

        finished = true;
        for node in &current_nodes {
            if !node.ends_with('Z') {
                finished = false;
                break;
            }
        }

        if finished {
            break;
        }
    }

    println!("Steps needed: {}", step);
}

pub fn part_two() {
    let input = fs::read_to_string("input").expect("Something went wrong reading the file");

    let mut split = input.split("\n\n");
    let instructions = split.next().unwrap().chars().collect::<Vec<char>>();
    let network_list = split
        .next()
        .unwrap()
        .split("\n")
        .map(|x| vec![&x[0..3], &x[7..10], &x[12..15]])
        .collect::<Vec<Vec<&str>>>();

    let network = network_list
        .iter()
        .map(|x| (x[0].to_string(), vec![x[1].to_string(), x[2].to_string()]))
        .collect::<HashMap<String, Vec<String>>>();

    let mut instruction_index = 0;
    let mut current_nodes = Vec::new();
    for key in network.keys() {
        if key.ends_with('A') {
            current_nodes.push(key.to_string());
        }
    }

    let mut steps = Vec::new();

    for mut node in current_nodes {
        let mut step: u64 = 0;

        while !node.ends_with('Z') {
            node = match instructions[instruction_index] {
                'L' => network[&node][0].clone(),
                'R' => network[&node][1].clone(),
                _ => panic!("Unknown instruction"),
            };

            instruction_index += 1;
            if instruction_index == instructions.len() {
                instruction_index = 0;
            }

            step += 1;
        }

        steps.push(step);
    }

    println!("Steps needed: {}", lcmx(&steps).unwrap());
}
