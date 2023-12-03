use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use uuid::Uuid;

/* Initial ideas:
I'm going to need to pull this into a 2d array, not sure what datatype that is
in Rust. I think I'll also read over each line to gather the index of the
beginning of any numbers. Combining these two pieces of information I should be
able to calculate the adjacent 2d array spaces and see if they are symbols or
not to determine if the number is applicable.
*/

fn main() {
    let input_file = "src/input.txt";

    // First gather the schematic size to efficiently initialize the vec array
    let mut engine_schematic_width: usize = 0;
    let mut engine_schematic_height: usize = 0;
    let mut i: usize = 0;

    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(ip) = line {
                if i == 1 {
                    engine_schematic_width = ip.len();
                }
                i += 1;
            }
        }
        engine_schematic_height = i;
    }

    // Capture the engine schematic in a 2d Vec array
    let mut engine_schematic = vec![vec![' '; engine_schematic_height]; engine_schematic_width];

    // Capture the numbers
    // The value is a Vec of usize containing row, column, number, length information.
    let mut numbers: Vec<Vec<usize>> = vec![vec![]];

    if let Ok(lines) = read_lines(input_file) {
        i = 0;
        let mut number_count = 0 as usize;
        for line in lines {
            if let Ok(ip) = line {
                // Set the engine schematic
                let line_array = ip.chars().collect::<Vec<char>>();
                engine_schematic[i] = line_array;

                // Set the numbers (if any)
                let mut last_character_digit = false;
                let mut current_character_index = 0 as usize;
                for character in ip.chars() {
                    if character.is_digit(10) {
                        if last_character_digit != true {
                            // replace all non numeric or . characters and grab
                            // the first match based on our window
                            //....90*12...
                            //....9012...

                            //    90 12
                            let re = Regex::new(r"[^0-9]").unwrap();
                            let number =
                                String::from(re.replace_all(&ip[current_character_index..], " "))
                                    .split(' ')
                                    .nth(0)
                                    .unwrap()
                                    .parse::<usize>()
                                    .unwrap();
                            numbers.push(vec![
                                i,
                                current_character_index,
                                number,
                                number.to_string().len(),
                            ]);
                            number_count += 1;
                        }
                        last_character_digit = true;
                    } else {
                        last_character_digit = false;
                    }
                    current_character_index += 1;
                }
                i += 1;
            }
        }
    }
    numbers.remove(0);

    let mut part_number_digits = HashMap::<[usize; 2], Uuid>::new();
    let mut part_number_lookup = HashMap::<Uuid, usize>::new();

    // Part 1
    // Loop over numbers
    let mut part_numbers_sum = 0 as usize;
    for number_info in numbers {
        let row = number_info[0];
        let column = number_info[1];

        let mut adjacent_values = Vec::<char>::new();
        // Loop over every digit in the number
        for digit in 0..number_info[3] {
            // Check to see if digit is a part number by checking adjacent values for a symbol

            let check_left = column + digit > 0;
            let check_up = row > 0;
            let check_right = column + digit < engine_schematic_width - 1;
            let check_down = row < engine_schematic_height - 1;

            if check_left {
                adjacent_values.push(engine_schematic[row][column + digit - 1]);
            }
            if check_left && check_up {
                adjacent_values.push(engine_schematic[row - 1][column + digit - 1]);
            }
            if check_up {
                adjacent_values.push(engine_schematic[row - 1][column + digit]);
            }
            if check_right && check_up {
                adjacent_values.push(engine_schematic[row - 1][column + digit + 1]);
            }
            if check_right {
                adjacent_values.push(engine_schematic[row][column + digit + 1]);
            }
            if check_right && check_down {
                adjacent_values.push(engine_schematic[row + 1][column + digit + 1]);
            }
            if check_down {
                adjacent_values.push(engine_schematic[row + 1][column + digit]);
            }
            if check_left && check_down {
                adjacent_values.push(engine_schematic[row + 1][column + digit - 1]);
            }
        }
        let re = Regex::new(r"[0-9.]").unwrap();
        let stripped_string =
            String::from(re.replace_all(&adjacent_values.iter().collect::<String>(), ""));
        if stripped_string.len() > 0 {
            // we have an adjacent symbol
            let part_number_uuid = Uuid::new_v4();
            part_number_lookup.insert(part_number_uuid, number_info[2]);
            for digit in 0..number_info[3] {
                part_number_digits.insert([row, column + digit], part_number_uuid);
            }

            part_numbers_sum += number_info[2]
        }
    }

    println!("The answer is {part_numbers_sum}");

    // Part 2
    let mut gear_ratio_sum: usize = 0;
    let mut potential_gears: Vec<[usize; 2]> = vec![];

    let mut current_row = 0 as usize;
    for row in engine_schematic {
        let mut current_index = 0 as usize;
        for character in row {
            if character == '*' {
                potential_gears.push([current_row, current_index])
            }
            current_index += 1;
        }
        current_row += 1;
    }

    for potential_gear in potential_gears {
        let mut adjacent_nodes: Vec<[usize; 2]> = vec![];
        let check_left = potential_gear[0] > 0;
        let check_up = potential_gear[0] > 0;
        let check_right = potential_gear[1] < engine_schematic_width - 1;
        let check_down = potential_gear[0] < engine_schematic_height - 1;

        if check_left {
            adjacent_nodes.push([potential_gear[0], potential_gear[1] - 1]);
        }
        if check_left && check_up {
            adjacent_nodes.push([potential_gear[0] - 1, potential_gear[1] - 1]);
        }
        if check_up {
            adjacent_nodes.push([potential_gear[0] - 1, potential_gear[1]]);
        }
        if check_right && check_up {
            adjacent_nodes.push([potential_gear[0] - 1, potential_gear[1] + 1]);
        }
        if check_right {
            adjacent_nodes.push([potential_gear[0], potential_gear[1] + 1]);
        }
        if check_right && check_down {
            adjacent_nodes.push([potential_gear[0] + 1, potential_gear[1] + 1]);
        }
        if check_down {
            adjacent_nodes.push([potential_gear[0] + 1, potential_gear[1]]);
        }
        if check_left && check_down {
            adjacent_nodes.push([potential_gear[0] + 1, potential_gear[1] - 1]);
        }

        let mut adjacent_part_number_uuids: Vec<Uuid> = vec![];
        for adjacent_node in adjacent_nodes {
            if part_number_digits.contains_key(&adjacent_node) {
                adjacent_part_number_uuids.push(part_number_digits[&adjacent_node])
            }
        }

        let unique_adjacent_part_number_uuids: Vec<Uuid> =
            adjacent_part_number_uuids.into_iter().unique().collect();
        let adjacent_part_number_count: usize = unique_adjacent_part_number_uuids.iter().count();

        if adjacent_part_number_count == 2 {
            // We have a gear
            gear_ratio_sum += part_number_lookup[&unique_adjacent_part_number_uuids[0]]
                * part_number_lookup[&unique_adjacent_part_number_uuids[1]]
        }
    }

    println!("The answer is {gear_ratio_sum}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
