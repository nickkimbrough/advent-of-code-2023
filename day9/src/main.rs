use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

/*
    Initial Thoughts:

    Part 1 they basically give the answer on how to check.... this terrifies me
    because I know they could make the patterns much harder for part 2.
    Nevertheless, I'll follow the solution given and calculate differences until
    they are 0 and then build it back up.
*/

fn main() {
    let input_file = "src/input.txt";
    let mut initial_sequences: Vec<Vec<i64>> = vec![];
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(ip) = line {
                initial_sequences.push(ip.split(' ').map(|f| f.parse::<i64>().unwrap()).collect());
            }
        }
    }

    // Part 1
    let mut generated_sequences: Vec<Vec<Vec<i64>>> = vec![];
    for sequence in initial_sequences.iter() {
        let mut sequence_steps: Vec<Vec<i64>> = vec![];
        sequence_steps.push(sequence.to_vec());

        'until_answer_found: loop {
            // If the last row is all zeroes, we found our answer
            let mut all_zeros = true;
            for step in sequence_steps.last().unwrap().iter() {
                if *step != 0 as i64 {
                    all_zeros = false;
                    break;
                }
            }
            if all_zeros {
                break 'until_answer_found;
            }

            // Get a peekable iterator to work with containing what's in the last row
            let mut last_steps = sequence_steps.last().unwrap().iter().peekable();
            // While our last_steps has more values
            let mut next_steps: Vec<i64> = vec![];
            while last_steps.peek() != None {
                let first_value = last_steps.next().unwrap();
                // only grab second value if it exists
                if last_steps.peek() != None {
                    let second_value = last_steps.peek().unwrap();
                    next_steps.push(*second_value - first_value);
                }
            }
            sequence_steps.push(next_steps);
        }
        generated_sequences.push(sequence_steps);
    }

    // We have the tree now, we need to extrapolate it
    // Start by adding a zero to the last row.
    let mut extrapolated_sequences: Vec<Vec<Vec<i64>>> = vec![];

    for generated_sequence in generated_sequences.iter_mut() {
        let mut extrapolated_sequence: Vec<Vec<i64>> = vec![];
        // let mut internal_generated_sequence =

        // Initialize it with the list of zeros
        let mut extrapolated_sequence_row: Vec<i64> = generated_sequence.last().unwrap().to_vec();
        extrapolated_sequence_row.push(0);
        extrapolated_sequence.push(extrapolated_sequence_row);

        for remaining_row in generated_sequence.iter_mut().rev().skip(1) {
            let last_number = *remaining_row.last().unwrap();
            let difference_number = *extrapolated_sequence.last().unwrap().last().unwrap();
            remaining_row.push(last_number + difference_number);
            extrapolated_sequence.push(remaining_row.to_vec());
        }
        extrapolated_sequences.push(extrapolated_sequence);
    }

    let answer: i64 = extrapolated_sequences
        .iter()
        .map(|x| *x.last().unwrap().last().unwrap())
        .sum();
    println!("Part 1 Answer: {answer}");

    // Part 2
    let mut extrapolated_sequences_2: Vec<Vec<Vec<i64>>> = vec![];

    for generated_sequence in generated_sequences.iter_mut() {
        let mut extrapolated_sequence: Vec<Vec<i64>> = vec![];

        let mut extrapolated_sequence_row: Vec<i64> = generated_sequence.last().unwrap().to_vec();
        extrapolated_sequence_row.push(0);
        extrapolated_sequence.push(extrapolated_sequence_row);

        for mut remaining_row in generated_sequence.iter_mut().rev().skip(1) {
            let first_number = *remaining_row.first().unwrap();
            let difference_number = *extrapolated_sequence.last().unwrap().first().unwrap();
            let mut new_remaining_row: Vec<i64> = vec![];
            new_remaining_row.push(first_number - difference_number);
            new_remaining_row.extend_from_slice(&remaining_row);

            remaining_row = &mut new_remaining_row;
            extrapolated_sequence.push(remaining_row.to_vec());
        }
        extrapolated_sequences_2.push(extrapolated_sequence);
    }

    let answer2: i64 = extrapolated_sequences_2
        .iter()
        .map(|x| *x.last().unwrap().first().unwrap())
        .sum();
    println!("Part 2 Answer: {answer2}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
