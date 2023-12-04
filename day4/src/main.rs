use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
   Initial Ideas and Thoughts:

   Data Gather
   For each card, I'll collect winning numbers and card numbers into a
   collection.

   Iterate & Analyze
   I'll create a sum of winning cards, and then iterate over every card.
   I'll create a winning number counter, then iterate over the
   card numbers to see if winning numbers contains that particular number. If
   so, I'll increment that counter by one. Then I'll use 2^(-1 + n) to get the
   score for that card. I'll then add it to the sum of winning cards.

   Part 2 probably has some sort of trap I'm not thinking of.
*/
fn main() {
    let input_file = "src/sampleinput.txt";

    if let Ok(lines) = read_lines(input_file) {
        let mut cards: HashMap<usize, [Vec<usize>; 2]> = HashMap::<usize, [Vec<usize>; 2]>::new();

        for line in lines {
            if let Ok(ip) = line {
                // Card 205: 54 17 93 26 35  9 61 49 81 42 | 94 14 76 52 15 18 38 41 69 28 16 31 73 32 47 37 71 23 82 90 33 75 24 85 11
                let card_id = ip
                    .replace("Card ", "")
                    .split(':')
                    .nth(0)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap() as usize;
                let winning_numbers: Vec<usize> = ip
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split('|')
                    .nth(0)
                    .unwrap()
                    .replace("  ", " ")
                    .split(" ")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                let card_numbers: Vec<usize> = ip
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split('|')
                    .nth(1)
                    .unwrap()
                    .replace("  ", " ")
                    .split(" ")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                cards.insert(card_id, [winning_numbers, card_numbers]);
            }
        }

        // Part 1
        let mut winning_card_score_sum: usize = 0;

        //for digit in 0..number_info[3] {

        for i in 1..cards.len() {
            let mut winning_number_count: usize = 0;
            let thing = &cards[&i];
            //Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            println!("Card {}: {} | {}", i, 2, 2);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
