use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
    Initial Thoughts:

    Part 1 they basically give the answer on how to check.... this terrifies me
    because I know they could make the patterns much harder for part 2.
    Nevertheless, I'll follow the solution given and calculate differences until
    they are 0 and then build it back up.
*/

fn main() {
    let input_file = "src/sampleinput.txt";
    let mut sequences: Vec<Vec<i64>> = vec![];
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(ip) = line {
                sequences.push(ip.split(' ').map(|f| f.parse::<i64>().unwrap()).collect());
            }
        }
    }

    // Part 1
    for sequence in sequences.iter() {
        let mut sequence_steps: Vec<Vec<i64>> = vec![];
        sequence_steps.push(sequence.to_vec());

        let mut answer_found = false;
        while !answer_found {
            for sequence_step in sequence_steps.iter() {
                // to do: build out the steps to 0
                //if sequence_step.sum() == 0 {}
            }
        }
    }
    println!("Balls");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
