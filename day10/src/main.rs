use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

fn main() {
    let input_file = "src/input.txt";
    let mut pipe_map: Vec<Vec<char>> = vec![];
    let mut pipe_map_width: usize = 0;
    let mut start_location: [usize; 2] = [0, 0];
    if let Ok(lines) = read_lines(input_file) {
        let mut line_counter = 0;
        for line in lines {
            if let Ok(ip) = line {
                let current_line = ip.chars().collect::<Vec<char>>();
                pipe_map.push(current_line);

                match ip
                    .chars()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .iter()
                    .position(|x| x == "S")
                {
                    Some(c) => {
                        pipe_map_width = ip.len() - 1;
                        start_location = [line_counter, c];
                    }
                    None => {}
                }
            }
            line_counter += 1;
        }
    }

    // Part 1
    // I think we can just loop through the entire pipe and divide the count by
    // 2 to get the answer.

    // First, lets get the connecting locations:
    // For the first one, lets just grab the first returned:
    let mut previous_location = start_location;
    let mut steps_taken = 1;
    let mut next_location = start_location;
    loop {
        let next_next_location = *get_next_pipe_locations(next_location, &pipe_map, pipe_map_width)
            .iter()
            .filter(|x| x != &&previous_location)
            .nth(0)
            .unwrap();
        previous_location = next_location;
        next_location = next_next_location;

        steps_taken += 1;
        if next_location == start_location {
            break;
        }
    }

    println!("Part 1: Steps Taken: {}", steps_taken / 2);
}

fn get_next_pipe_locations(
    current_location: [usize; 2],
    pipe_map: &Vec<Vec<char>>,
    pipe_map_width: usize,
) -> Vec<[usize; 2]> {
    let mut next_pipe_locations: Vec<[usize; 2]> = vec![];

    let current_value = pipe_map[current_location[0]][current_location[1]];
    let check_left = current_location[1] > 0 && ['-', 'J', '7', 'S'].contains(&current_value);
    let check_up = current_location[0] > 0 && ['|', 'L', 'J', 'S'].contains(&current_value);
    let check_right =
        current_location[1] < pipe_map_width && ['-', 'L', 'F', 'S'].contains(&current_value);
    let check_down =
        current_location[0] < pipe_map_width && ['|', '7', 'F', 'S'].contains(&current_value);

    if check_left {
        let left_connectors = ['-', 'L', 'F', 'S'];
        let left_value = &pipe_map[current_location[0]][current_location[1] - 1];
        if left_connectors.contains(left_value) {
            next_pipe_locations.push([current_location[0], current_location[1] - 1]);
        }
    }
    if check_up {
        let up_connectors = ['|', '7', 'F', 'S'];
        let up_value = &pipe_map[current_location[0] - 1][current_location[1]];
        if up_connectors.contains(up_value) {
            next_pipe_locations.push([current_location[0] - 1, current_location[1]]);
        }
    }
    if check_right {
        let right_connectors = ['-', '7', 'J', 'S'];
        let right_value = &pipe_map[current_location[0]][current_location[1] + 1];
        if right_connectors.contains(right_value) {
            next_pipe_locations.push([current_location[0], current_location[1] + 1]);
        }
    }
    if check_down {
        let down_connectors = ['|', 'L', 'J', 'S'];
        let down_value = &pipe_map[current_location[0] + 1][current_location[1]];
        if down_connectors.contains(down_value) {
            next_pipe_locations.push([current_location[0] + 1, current_location[1]]);
        }
    }

    return next_pipe_locations;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
