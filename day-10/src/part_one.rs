use std::fs;

const INF: u32 = 1_000_000_000;

pub fn part_one() {
    let input = fs::read_to_string("input").expect("Something went wrong reading the file");
    let input_array = input
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (mut start_row, mut start_col) = (0, 0);
    let mut connected_nodes = Vec::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start_row = i;
                start_col = j;

                if i > 0 && ['|', '7', 'F'].contains(&input_array[i - 1][j]) {
                    connected_nodes.push((i - 1, j));
                }
                if i < input_array.len() - 1 && ['|', 'J', 'L'].contains(&input_array[i + 1][j]) {
                    connected_nodes.push((i + 1, j));
                }
                if j > 0 && ['-', 'F', 'L'].contains(&input_array[i][j - 1]) {
                    connected_nodes.push((i, j - 1));
                }
                if j < input_array[i].len() - 1 && ['-', 'J', '7'].contains(&input_array[i][j + 1])
                {
                    connected_nodes.push((i, j + 1));
                }

                break;
            }
        }
    }

    println!("Start row: {}, start col: {}", start_row, start_col);
    println!("Connected nodes: {:?}", connected_nodes);

    let mut distances = vec![vec![INF; input_array[0].len()]; input_array.len()];

    for connected_node in connected_nodes {
        let mut prev_node = (start_row, start_col);
        let mut curr_node = connected_node;

        println!(" ---- Node: {:?} ----", curr_node);

        let mut distance = 1;
        distances[curr_node.0][curr_node.1] = distance;

        while curr_node != (start_row, start_col) {
            let (i, j) = curr_node;
            let mut next_node = (0, 0);

            if i > 0
                && (i - 1, j) != prev_node
                && ['|', 'L', 'J'].contains(&input_array[i][j])
                && ['|', '7', 'F', 'S'].contains(&input_array[i - 1][j])
            {
                next_node = (i - 1, j);
            }
            if i < input_array.len() - 1
                && (i + 1, j) != prev_node
                && ['|', '7', 'F'].contains(&input_array[i][j])
                && ['|', 'J', 'L', 'S'].contains(&input_array[i + 1][j])
            {
                next_node = (i + 1, j);
            }
            if j > 0
                && (i, j - 1) != prev_node
                && ['-', 'J', '7'].contains(&input_array[i][j])
                && ['-', 'F', 'L', 'S'].contains(&input_array[i][j - 1])
            {
                next_node = (i, j - 1);
            }
            if j < input_array[i].len() - 1
                && (i, j + 1) != prev_node
                && ['-', 'F', 'L'].contains(&input_array[i][j])
                && ['-', 'J', '7', 'S'].contains(&input_array[i][j + 1])
            {
                next_node = (i, j + 1);
            }

            if next_node == (start_row, start_col) {
                break;
            }
            assert!(next_node != (0, 0), "No next node found");

            prev_node = curr_node;
            curr_node = next_node;

            distance += 1;
            if distance < distances[curr_node.0][curr_node.1] {
                distances[curr_node.0][curr_node.1] = distance;
            }

            println!("Moved to node {}, {}", curr_node.0 + 1, curr_node.1 + 1);
        }
    }

    for i in 0..distances[0].len() {
        for j in 0..distances.len() {
            if distances[i][j] == INF {
                distances[i][j] = 0;
            }
        }
    }

    println!(
        "Max distance: {}",
        distances
            .iter()
            .map(|x| x.iter().max().unwrap())
            .max()
            .unwrap()
    );
}

// pub fn part_one() {
//     let input = fs::read_to_string("input").expect("Something went wrong reading the file");
//     let input_array = input
//         .split("\n")
//         .map(|x| x.chars().collect::<Vec<char>>())
//         .collect::<Vec<Vec<char>>>();

//     for (i, line) in input.lines().enumerate() {
//         for (j, c) in line.chars().enumerate() {
//             let node_type = match c {
//                 '.' => continue,
//                 '|' => NodeType::TopToBottom,
//                 '-' => NodeType::LeftToRight,
//                 'J' => NodeType::LeftToTop,
//                 '7' => NodeType::LeftToBottom,
//                 'L' => NodeType::RightToTop,
//                 'F' => NodeType::RightToBottom,
//                 'S' => {
//                     let top = i > 0 && ['|', '7', 'F'].contains(&input_array[i - 1][j]);
//                     let bottom = i < input_array.len() - 1
//                         && ['|', 'J', 'L'].contains(&input_array[i + 1][j]);
//                     let left = j > 0 && ['-', 'F', 'L'].contains(&input_array[i][j - 1]);
//                     let right = j < input_array[i].len() - 1
//                         && ['-', 'J', '7'].contains(&input_array[i][j + 1]);

//                     if top && bottom && !left && !right {
//                         NodeType::TopToBottom
//                     } else if !top && !bottom && left && right {
//                         NodeType::LeftToRight
//                     } else if top && !bottom && left && !right {
//                         NodeType::LeftToTop
//                     } else if !top && bottom && left && !right {
//                         NodeType::LeftToBottom
//                     } else if top && !bottom && !left && right {
//                         NodeType::RightToTop
//                     } else if !top && bottom && !left && right {
//                         NodeType::RightToBottom
//                     } else {
//                         panic!("Can't determine node type at row {} and column {}", i, j)
//                     }
//                 }
//                 _ => panic!("Unexpected character: {}", c),
//             };
//         }
//     }
// }
