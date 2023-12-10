use std::collections::HashMap;
use std::fs;

pub fn part_one() {
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
        .map(|x| (x[0], vec![x[1], x[2]]))
        .collect::<HashMap<&str, Vec<&str>>>();

    let mut instruction_index = 0;
    let mut current_node = "AAA";

    let mut step: u64 = 0;

    while current_node != "ZZZ" {
        match instructions[instruction_index] {
            'L' => current_node = network[current_node][0],
            'R' => current_node = network[current_node][1],
            _ => panic!("Unknown instruction"),
        }

        instruction_index += 1;
        if instruction_index == instructions.len() {
            instruction_index = 0;
        }

        step += 1;
    }

    println!("Steps needed: {}", step);
}
