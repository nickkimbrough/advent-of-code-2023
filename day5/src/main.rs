use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
   Initial thoughts:
   Looks like we got a one way mapping chain like this:
   seed -> soil -> fertilizer -> water -> light -> temperature -> humidity -> location

   Efficiency is going to be important here. I think I can collect a list of
   maps (it's own struct?) in order and then iterate over them to get the final
   output, as the functionality of the maps doesn't really change, just their
   properties. I'll add a method to the struct to get an output from an input.
*/
fn main() {
    let input_file = "src/input.txt";

    let mut maps: Vec<AlmanacMap> = vec![];
    let mut seeds: Vec<usize> = vec![];

    if let Ok(lines) = read_lines(input_file) {
        let mut source_maps: Vec<[usize; 3]> = vec![];
        let mut map_id_counter: usize = 0;
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with("seeds: ") {
                    seeds = ip
                        .replace("seeds: ", "")
                        .split(" ")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                } else if ip.ends_with(":") {
                    if source_maps.len() > 0 {
                        let new_map: AlmanacMap =
                            AlmanacMap::new(map_id_counter, source_maps.clone());
                        maps.push(new_map);
                        map_id_counter += 1;
                    }
                    source_maps.clear();
                } else if ip.len() > 0 {
                    let map_numbers: Vec<usize> = ip
                        .split(" ")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    source_maps.push([map_numbers[0], map_numbers[1], map_numbers[2]]);
                }
            }
        }
        let new_map: AlmanacMap = AlmanacMap::new(map_id_counter, source_maps.clone());
        maps.push(new_map);
    }

    // Part 1
    let mut final_locations: Vec<usize> = vec![];
    for seed in seeds.iter() {
        let mut final_location = *seed;
        for map in &maps {
            final_location = map.get_destination(final_location);
        }
        final_locations.push(final_location);
    }

    let answer = final_locations.iter().min().unwrap().to_string();
    println!("{answer}");

    // Part 2
    let mut seeds_part_2: Vec<usize> = vec![];
    while seeds.len() > 0 {
        let range = seeds.pop().unwrap();
        let seed = seeds.pop().unwrap();
        seeds_part_2.append(&mut (seed..seed + range).collect());
    }

    let mut part_2_answer: usize = usize::MAX;
    let mut previous_solutions: HashMap<[usize; 2], usize> = HashMap::new();
    for seed in seeds_part_2.iter() {
        let mut final_location = *seed;
        let mut current_steps: Vec<[usize; 2]> = vec![];

        for map in &maps {
            current_steps.push([final_location, map.id]);
            let potential_solution = [final_location, map.id];
            if previous_solutions.contains_key(&potential_solution) {
                final_location = *previous_solutions.get(&potential_solution).unwrap();
                break;
            }
            final_location = map.get_destination(final_location);
        }
        for current_step in current_steps {
            previous_solutions.insert(current_step, final_location);
        }
        if final_location < part_2_answer {
            part_2_answer = final_location;
        }
    }

    println!("{part_2_answer}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct AlmanacMap {
    id: usize,
    source_maps: Vec<[usize; 3]>,
}

impl AlmanacMap {
    fn default() -> AlmanacMap {
        AlmanacMap {
            id: 0,
            source_maps: vec![],
        }
    }

    pub fn new(id: usize, source_maps: Vec<[usize; 3]>) -> Self {
        let mut new_almanac_map: AlmanacMap = AlmanacMap::default();

        new_almanac_map.id = id;
        new_almanac_map.source_maps = source_maps;

        return new_almanac_map;
    }

    pub fn get_destination(&self, input: usize) -> usize {
        for source_map in self.source_maps.iter() {
            match (source_map[1]..source_map[1] + source_map[2]).contains::<usize>(&input) {
                true => {
                    return input - source_map[1] + source_map[0];
                }
                false => continue,
            }
        }
        return input;
    }
}
