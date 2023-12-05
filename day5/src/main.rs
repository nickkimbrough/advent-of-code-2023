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
                        let new_map: AlmanacMap = AlmanacMap::new(source_maps.clone());
                        maps.push(new_map)
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
        let new_map: AlmanacMap = AlmanacMap::new(source_maps.clone());
        maps.push(new_map);
    }

    // Part 1
    let mut final_locations: Vec<usize> = vec![];
    for seed in seeds {
        let mut final_location = seed;
        for map in &maps {
            final_location = map.get_destination(final_location);
        }
        final_locations.push(final_location);
    }

    let answer = final_locations.iter().min().unwrap().to_string();
    println!("{answer}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct AlmanacMap {
    source_maps: Vec<[usize; 3]>,
    source_map_mappings: HashMap<usize, usize>,
}

impl AlmanacMap {
    fn default() -> AlmanacMap {
        AlmanacMap {
            source_maps: vec![],
            source_map_mappings: HashMap::new(),
        }
    }

    pub fn new(source_maps: Vec<[usize; 3]>) -> Self {
        let mut new_almanac_map: AlmanacMap = AlmanacMap::default();

        new_almanac_map.source_maps = source_maps;
        new_almanac_map.get_mappings();

        return new_almanac_map;
    }

    pub fn get_destination(&self, input: usize) -> usize {
        match self.source_map_mappings.contains_key(&input) {
            true => {
                return self.source_map_mappings[&input];
            }
            false => return input,
        }
    }

    fn get_mappings(&mut self) {
        let mut mappings: HashMap<usize, usize> = HashMap::new();
        for source_map in &self.source_maps {
            for i in 0..source_map[2] {
                mappings.insert(source_map[1] + i, source_map[0] + i);
            }
        }
        self.source_map_mappings = mappings;
    }
}
