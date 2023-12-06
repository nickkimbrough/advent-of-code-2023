use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
    Initial thoughts:

    It feels like there's probably a formula here to graph the distance
    given time held which we could then get values that produce results greater
    than the record.

    At least for part one, I will gather the records and just do an iteration
    applying the above formula and then getting the answer. It seems simple on
    the surface.
*/

fn main() {
    let input_file = "src/input.txt";
    let mut races: Vec<[usize; 2]> = vec![];
    let mut times: Vec<usize> = vec![];
    let mut distances: Vec<usize> = vec![];

    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(ip) = line {
                let values: Vec<usize> = ip
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .trim()
                    .split(' ')
                    .filter(|x| x.len() > 0)
                    .map(|x| x.trim().parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                match ip.starts_with("Time:") {
                    true => times = values,
                    false => distances = values,
                }
            }
        }
        for i in 0..times.len() {
            races.push([times[i], distances[i]]);
        }
    }

    // part 1
    let mut answer: usize = 1;
    for race in races {
        let mut time_remaining = race[0];
        let mut time_held: usize = 0;
        let record_distance = race[1];
        let mut ways_to_win: usize = 0;

        while time_remaining > 0 {
            time_held += 1;
            time_remaining -= 1;
            if time_held * time_remaining > record_distance {
                ways_to_win += 1;
            }
        }

        answer = answer * ways_to_win;
    }

    println!("{answer}");

    // part 2
    let mut race_time: usize = times
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let mut time_held2: usize = 0;
    let race_record: usize = distances
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let mut answer2: usize = 0;

    while race_time > 0 {
        time_held2 += 1;
        race_time -= 1;
        if time_held2 * race_time > race_record {
            answer2 += 1;
        }
    }

    println!("{answer2}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
