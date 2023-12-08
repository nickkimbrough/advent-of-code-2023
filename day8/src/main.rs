use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;

/*
    Initial Thoughts:

    I think using a HashMap<String, [String; 2]> to gather all of the nodes
    might be what makes sense here. This feels like some sort of tree that I
    could build to figure it out, but for now I'm going to try to just gather
    the data and follow the steps until ZZZ is reached by iterating.
*/

fn main() {
    let input_file = "src/input.txt";
    let mut directions: Vec<String> = vec![];
    let mut map_nodes: HashMap<String, [String; 2]> = HashMap::new();
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(ip) = line {
                if !ip.contains('=') && ip.len() > 0 {
                    directions = ip
                        .graphemes(true)
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>();
                } else if ip.contains('=') {
                    let node = ip.graphemes(true).take(3).collect::<String>();
                    let element1 = ip.graphemes(true).skip(7).take(3).collect::<String>();
                    let element2 = ip.graphemes(true).skip(12).take(3).collect::<String>();
                    map_nodes.insert(node, [element1, element2]);
                }
            }
        }
    }

    // Part 1 (comment out for part 2 and vice versa, bad rules are bad)
    // let mut steps_taken: usize = 0;
    // let mut current_node: String = "AAA".to_string();

    // 'outer: loop {
    //     for direction in directions.iter() {
    //         let next_node = match direction.as_str() {
    //             "L" => &map_nodes.get(&current_node).unwrap()[0],
    //             "R" => &map_nodes.get(&current_node).unwrap()[1],
    //             _ => panic!("Bad Directions!"),
    //         };

    //         steps_taken += 1;

    //         if next_node == "ZZZ" {
    //             break 'outer;
    //         }

    //         current_node = next_node.to_string();
    //     }
    // }

    // println!("Step 1 steps taken: {steps_taken}");

    // Part 2
    let mut steps_to_z: Vec<usize> = vec![];

    let mut current_nodes = map_nodes
        .iter()
        .filter(|x| x.0.ends_with('A'))
        .map(|x| x.0.to_string())
        .collect::<Vec<String>>();

    for current_node in current_nodes.iter() {
        let mut inner_node = current_node.to_string();
        let mut steps_taken_2: usize = 0;
        'outer: loop {
            for direction in directions.iter() {
                let next_node = match direction.as_str() {
                    "L" => &map_nodes.get(&inner_node).unwrap()[0],
                    "R" => &map_nodes.get(&inner_node).unwrap()[1],
                    _ => panic!("Bad Directions!"),
                };

                steps_taken_2 += 1;

                if next_node.ends_with("Z") {
                    break 'outer;
                }

                inner_node = next_node.to_string();
            }
        }
        steps_to_z.push(steps_taken_2);
    }

    let part_2_answer = lcm(&steps_to_z);

    println!("Step 2 steps taken: {part_2_answer}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// LCM algorithms taken from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
