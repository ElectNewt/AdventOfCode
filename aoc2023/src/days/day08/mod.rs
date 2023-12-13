use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Map {
    order: Vec<char>,
    nodes: HashMap<String, Node>,
}

#[derive(Debug, Clone)]
pub struct Node {
    left: String,
    right: String,
}

pub fn task_one(content: String) -> String {
    println!("Day 8! task 1");
    let map = file_to_map(content);
    let mut current_position = "AAA".to_string();
    let mut steps = 0;

    while (1 == 1) {
        for letter in map.order.clone() {
            steps = steps + 1;

            let value = map.nodes.get(&current_position);
            if let Some(value) = value {
                if letter == 'L' {
                    current_position = value.left.clone();
                } else if letter == 'R' {
                    current_position = value.right.clone();
                }
            } else {
                panic!("Something is wrong");
            }

            if current_position == "ZZZ" {
                return steps.to_string();
            }
        }
    }


    return steps.to_string();
}


pub fn task_two(content: String) -> String {
    println!("Day 8! task 2");
    let map = file_to_map(content);

    let mut current_positions: Vec<&String> = map.nodes.keys()
        .filter(|key| key.ends_with('A') )
        .collect();

    let how_many_starting_positions = current_positions.clone().iter().count();

    let mut steps: u64 = 0;

    while (1 == 1) {
        for letter in map.order.clone() {
            steps = steps + 1;
            //println!("step  ---------- {}", steps);

            for iterator in 0..how_many_starting_positions {
                 let current_position = current_positions[iterator];
                let value = map.nodes.get(current_position);
                if let Some(value) = value {
                    if letter == 'L' {
                        current_positions[iterator] = &value.left;
                    } else if letter == 'R' {
                        current_positions[iterator] = &value.right;
                    }
                } else {
                    panic!("Something is wrong");
                }
            }



            if current_positions.iter().all(|s| s.ends_with('Z')) {
                return steps.to_string();
            }
        }
    }


    return steps.to_string();
}

fn file_to_map(content: String) -> Map {
    //for some reason \n\n is not working :(
    let groups: Vec<_> = content.split("\n").collect();
    let order: Vec<char> = groups[0].chars()
        .filter(|&c| !c.is_whitespace())
        .collect();

    let mut nodes: HashMap<String, Node> = HashMap::new();
    let many_groups = groups.iter().count();

    for iterator in 2..many_groups {
        let inside_line = groups[iterator].split_once("=").unwrap();
        let key = inside_line.0.trim().to_string();

        let replaced = inside_line.1.replace('(', "").replace(')', "").trim().to_string();
        let right_side = replaced.split_once(",").unwrap();


        let node = Node {
            right: right_side.1.trim().to_string(),
            left: right_side.0.trim().to_string(),
        };
        nodes.insert(key, node);
    }
    return Map {
        order,
        nodes,
    };
}