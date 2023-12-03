use std::cmp;

use std::fs;

#[derive(Debug)]
struct PartNumber {
    part: usize,
    line: usize,
    start: usize,
    end: usize,
    valid: bool,
}

fn part_1() {
    let input = fs::read_to_string("input").expect("Error reading input file");

    let schematic = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (rows, cols) = (schematic.len(), schematic[0].len());

    let mut parts_sum = 0;

    for (l_idx, line) in input.lines().enumerate() {
        let mut reading_part = false;
        let mut done_reading_part = false;

        let mut part = Vec::new();
        let mut start = 0;

        for (c_idx, character) in line.chars().enumerate() {
            if character.is_digit(10) {
                if !reading_part {
                    reading_part = true;
                    start = c_idx;
                }
                part.push(character);

                if c_idx == cols - 1 {
                    done_reading_part = true;
                    reading_part = false;
                }
            } else if reading_part {
                done_reading_part = true;
                reading_part = false;
            }

            if done_reading_part {
                let lmin = if l_idx == 0 { 0 } else { l_idx - 1 };
                let lmax = if l_idx + 1 < rows { l_idx + 2 } else { rows };
                let cmin = if start == 0 { 0 } else { start - 1 };
                let cmax = if c_idx < cols { c_idx + 1 } else { cols };

                let mut valid = false;
                for i in lmin..lmax {
                    for j in cmin..cmax {
                        if schematic[i][j] != '.' && !schematic[i][j].is_digit(10) {
                            valid = true;
                        }
                    }
                }

                let part_number = part.iter().collect::<String>().parse::<usize>().unwrap();
                if valid {
                    parts_sum += part_number;
                }

                part = Vec::new();
                done_reading_part = false;
            }
        }
    }

    println!("Parts sum: {}", parts_sum);
}

fn part_2() {
    let input = fs::read_to_string("input").expect("Error reading input file");

    let schematic = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (rows, cols) = (schematic.len(), schematic[0].len());

    let mut parts = Vec::new();
    let mut parts_sum = 0;

    for (l_idx, line) in input.lines().enumerate() {
        let mut reading_part = false;
        let mut done_reading_part = false;

        let mut part = Vec::new();
        let mut start = 0;

        for (c_idx, character) in line.chars().enumerate() {
            if character.is_digit(10) {
                if !reading_part {
                    reading_part = true;
                    start = c_idx;
                }
                part.push(character);

                if c_idx == cols - 1 {
                    done_reading_part = true;
                    reading_part = false;
                }
            } else if reading_part {
                done_reading_part = true;
                reading_part = false;
            }

            if done_reading_part {
                let lmin = if l_idx == 0 { 0 } else { l_idx - 1 };
                let lmax = if l_idx + 1 < rows { l_idx + 2 } else { rows };
                let cmin = if start == 0 { 0 } else { start - 1 };
                let cmax = if c_idx < cols { c_idx + 1 } else { cols };

                let mut valid = false;
                for i in lmin..lmax {
                    for j in cmin..cmax {
                        if schematic[i][j] != '.' && !schematic[i][j].is_digit(10) {
                            valid = true;
                        }
                    }
                }

                let part_number = part.iter().collect::<String>().parse::<usize>().unwrap();
                if valid {
                    parts_sum += part_number;
                }

                parts.push(PartNumber {
                    part: part_number,
                    line: l_idx,
                    start: start,
                    end: c_idx - 1,
                    valid: valid,
                });

                part = Vec::new();
                done_reading_part = false;
            }
        }
    }

    parts_sum = 0;

    for (l_idx, line) in input.lines().enumerate() {
        for (c_idx, character) in line.chars().enumerate() {
            if character == '*' {
                let lmin = if l_idx == 0 { 0 } else { l_idx - 1 };
                let lmax = if l_idx + 1 < rows { l_idx + 1 } else { rows };
                let cmin = if c_idx == 0 { 0 } else { c_idx - 1 };
                let cmax = if c_idx < cols { c_idx + 1 } else { cols };

                let mut geared_parts = Vec::new();
                for part in &parts {
                    if lmin <= part.line
                        && part.line <= lmax
                        && ((cmin <= part.start && part.start <= cmax)
                            || (cmin <= part.end && part.end <= cmax))
                    {
                        geared_parts.push(part);
                    }
                }

                if geared_parts.len() == 2 {
                    parts_sum += geared_parts[0].part * geared_parts[1].part;
                }
            }
        }
    }

    println!("Parts sum: {}", parts_sum);
}

fn main() {
    part_1();
    part_2();
}
