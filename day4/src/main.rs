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
    let input_file = "src/input.txt";

    if let Ok(lines) = read_lines(input_file) {
        let mut cards: Vec<Card> = vec![];

        for line in lines {
            if let Ok(ip) = line {
                let card_id = ip
                    .replace("Card", "")
                    .replace(" ", "")
                    .split(':')
                    .nth(0)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap() as usize;
                let winning_numbers = ip
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split('|')
                    .nth(0)
                    .unwrap()
                    .replace("  ", " ")
                    .trim()
                    .split(' ')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                let card_numbers: Vec<usize> = ip
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split('|')
                    .nth(1)
                    .unwrap()
                    .replace("  ", " ")
                    .trim()
                    .split(' ')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                cards.push(Card::new(card_id, winning_numbers, card_numbers));
            }
        }

        // Part 1
        let winning_card_score_sum: usize = cards.iter().map(|x| x.part_1_score).sum();
        println!("Answer: {winning_card_score_sum}");

        // Part 2
        let initial_card_count: usize = cards.len();
        loop {
            let mut card_numbers_to_duplicate: Vec<usize> = vec![];
            for card in cards.iter_mut() {
                if !card.processed {
                    for i in 1..card.winning_numbers_count + 1 {
                        if card.number + i <= initial_card_count {
                            card_numbers_to_duplicate.push(card.number + i);
                        }
                    }
                    card.processed = true;
                }
            }

            if card_numbers_to_duplicate.len() > 0 {
                for card_number_to_duplicate in card_numbers_to_duplicate {
                    let card_to_duplicate = cards
                        .iter()
                        .filter(|x| x.number == card_number_to_duplicate)
                        .nth(0)
                        .unwrap();

                    cards.push(Card::new(
                        card_number_to_duplicate,
                        card_to_duplicate.winners.to_vec(),
                        card_to_duplicate.card_numbers.to_vec(),
                    ));
                }
            } else {
                break;
            }
        }
        println!("Answer: {}", cards.len());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Card {
    number: usize,
    winners: Vec<usize>,
    card_numbers: Vec<usize>,
    winning_numbers_count: usize,
    part_1_score: usize,
    processed: bool,
}

impl Card {
    pub fn new(number: usize, winners: Vec<usize>, card_numbers: Vec<usize>) -> Self {
        let winning_numbers_count = card_numbers
            .iter()
            .map(|x| match winners.contains(x) {
                true => 1,
                _ => 0,
            })
            .sum();

        let part_1_score = match winning_numbers_count > 0 {
            true => usize::pow(2, winning_numbers_count as u32 - 1),
            false => 0,
        };

        Card {
            number,
            winners,
            card_numbers,
            winning_numbers_count,
            part_1_score,
            processed: false,
        }
    }
}
