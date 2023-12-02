use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Read the file and iterate over every line
    let mut games: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
    if let Ok(lines) = read_lines("src/input.txt") {
        let mut i: usize = 1;
        for line in lines {
            if let Ok(ip) = line {
                // Gather all the information into the HashMap for ease of processing
                let handful_start_index = ip.chars().position(|x| x == ':').unwrap() + 2;
                let handfuls = ip[handful_start_index..].split(';');

                let mut handful_results: Vec<Vec<usize>> = vec![];

                let mut j: usize = 0;
                for handful in handfuls {
                    let cube_colors = handful.split(',');

                    let mut red: usize = 0;
                    let mut green: usize = 0;
                    let mut blue: usize = 0;
                    for cube_color in cube_colors {
                        if cube_color.trim().ends_with("red") {
                            red = cube_color
                                .trim()
                                .split(' ')
                                .nth(0)
                                .unwrap()
                                .parse()
                                .unwrap();
                        }
                        if cube_color.trim().ends_with("green") {
                            green = cube_color
                                .trim()
                                .split(' ')
                                .nth(0)
                                .unwrap()
                                .parse()
                                .unwrap();
                        }
                        if cube_color.trim().ends_with("blue") {
                            blue = cube_color
                                .trim()
                                .split(' ')
                                .nth(0)
                                .unwrap()
                                .parse()
                                .unwrap();
                        }
                    }

                    let vec1 = vec![red, green, blue];
                    handful_results.insert(j, vec1);
                    j += 1;
                }

                games.insert(i, handful_results);
            }
            i += 1;
        }
    }

    // part 1
    let red_cubes: usize = 12;
    let green_cubes: usize = 13;
    let blue_cubes: usize = 14;
    let mut possible_game_sums_part_1: usize = 0;

    for (game_id, handfuls) in &games {
        let mut possible_game = true;
        for handful in handfuls {
            if handful[0] > red_cubes {
                possible_game = false;
            }
            if handful[1] > green_cubes {
                possible_game = false;
            }
            if handful[2] > blue_cubes {
                possible_game = false;
            }
        }
        if possible_game {
            possible_game_sums_part_1 += game_id;
        }
    }
    println!("Part 1 Answer: {possible_game_sums_part_1}");

    // part 2
    let mut possible_game_powers_sums_part_2: usize = 0;

    for (_game_id, handfuls) in &games {
        let mut maximum_red: usize = 0;
        let mut maximum_green: usize = 0;
        let mut maximum_blue: usize = 0;

        for handful in handfuls {
            if handful[0] > maximum_red {
                maximum_red = handful[0];
            }
            if handful[1] > maximum_green {
                maximum_green = handful[1];
            }
            if handful[2] > maximum_blue {
                maximum_blue = handful[2];
            }
        }
        possible_game_powers_sums_part_2 += maximum_red * maximum_green * maximum_blue;
    }
    println!("{possible_game_powers_sums_part_2}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
