use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
   Initial thoughts:

   Could I use a different base to easily calculate hand values here? It should
   be possible to calculate every possible hand value and assign it a unique
   score.
*/

fn main() {
    let input_file = "src/input.txt";
    let mut hands: Vec<CamelCardsHand> = vec![];
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(ip) = line {
                let hand: Vec<usize> = ip
                    .split(' ')
                    .nth(0)
                    .unwrap()
                    .chars()
                    .map(|f| match f {
                        'A' => 14,
                        'K' => 13,
                        'Q' => 12,
                        'J' => 11,
                        'T' => 10,
                        _ => f.to_string().parse::<usize>().unwrap(),
                    })
                    .collect();
                let bid: usize = ip.split(' ').nth(1).unwrap().parse::<usize>().unwrap();
                hands.push(CamelCardsHand::new(hand, bid));
            }
        }
    }

    //Part 1
    hands.sort_by_key(|x| (x.tier_score, x.hand_score));

    let mut iterator: usize = 1;
    let mut answer: usize = 0;
    for hand in hands.iter() {
        answer += hand.bid * iterator;
        iterator += 1;
    }

    println!("{answer}");

    //Part 2
    hands.sort_by_key(|x| (x.joker_tier_score, x.joker_hand_score));

    let mut iterator2: usize = 1;
    let mut answer2: usize = 0;
    for hand in hands.iter() {
        answer2 += hand.bid * iterator2;
        iterator2 += 1;
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

struct CamelCardsHand {
    hand: Vec<usize>,
    bid: usize,
    hand_score: usize,
    tier_score: usize,
    joker_hand_score: usize,
    joker_tier_score: usize,
}

impl CamelCardsHand {
    fn default() -> CamelCardsHand {
        CamelCardsHand {
            hand: vec![],
            bid: usize::default(),
            hand_score: usize::default(),
            tier_score: usize::default(),
            joker_hand_score: usize::default(),
            joker_tier_score: usize::default(),
        }
    }

    pub fn new(hand: Vec<usize>, bid: usize) -> Self {
        let mut new_camel_cards_hand: CamelCardsHand = CamelCardsHand::default();

        new_camel_cards_hand.hand = hand;
        new_camel_cards_hand.bid = bid;
        new_camel_cards_hand.set_card_score();
        new_camel_cards_hand.set_joker_card_score();

        return new_camel_cards_hand;
    }

    fn set_card_score(&mut self) {
        let mut tier_value: usize = 0;
        let mut hand_value: usize = 0;
        let lkadfjs = self.hand.clone();
        let mut handz: HashMap<usize, usize> = HashMap::new();

        let mut i = 0;
        for hand in lkadfjs.iter().rev() {
            match handz.contains_key(hand) {
                true => handz.insert(*hand, handz[hand] + 1),
                false => handz.insert(*hand, 1),
            };
            hand_value += 10_usize.pow(i) * hand;
            i += 4;
        }

        // get the hand tier
        if handz.values().any(|&x| x == 5) {
            tier_value = 7;
        } else if handz.values().any(|&x| x == 4) {
            tier_value = 6;
        } else if handz.values().any(|&x| x == 3) && handz.values().any(|&x| x == 2) {
            tier_value = 5;
        } else if handz.values().any(|&x| x == 3) {
            tier_value = 4;
        } else if handz.values().any(|&x| x == 2) {
            if handz.values().filter(|&&x| x == 2).count() == 2 {
                tier_value = 3;
            } else {
                tier_value = 2;
            }
        } else {
            tier_value = 1;
        }

        self.hand_score = hand_value;
        self.tier_score = tier_value;
    }

    fn set_joker_card_score(&mut self) {
        let mut tier_value: usize = 0;
        let mut hand_value: usize = 0;
        let hand_clone = self.hand.clone();
        let mut hands: HashMap<usize, usize> = HashMap::new();

        let mut i = 0;
        for mut hand in hand_clone.iter().rev() {
            if hand == &11_usize {
                hand = &1;
                hand_value += 10_usize.pow(i) * hand;
                i += 2;
                continue;
            }
            match hands.contains_key(hand) {
                true => hands.insert(*hand, hands[hand] + 1),
                false => hands.insert(*hand, 1),
            };
            hand_value += 10_usize.pow(i) * hand;
            i += 2;
        }

        // get the hand tier
        let joker_count = hand_clone.iter().filter(|&&x| x == 11).count();

        if hands.values().any(|&x| x >= 5 - joker_count) || joker_count == 5 {
            tier_value = 7;
        } else if hands.values().any(|&x| x >= 4 - joker_count) || joker_count == 4 {
            tier_value = 6;
        } else if hands.values().filter(|&&x| x == 2).count() == 2 && joker_count == 1 {
            tier_value = 5;
        } else if hands.values().any(|&x| x == 3)
            && hands.values().any(|&x| x == 2)
            && joker_count == 0
        {
            tier_value = 5;
        } else if hands.values().any(|&x| x >= 3 - joker_count) || joker_count == 3 {
            tier_value = 4;
        } else if hands.values().any(|&x| x >= 2 - joker_count) || joker_count == 2 {
            if hands.values().filter(|&&x| x == 2).count() == 2
                || hands.values().filter(|&&x| x == 2).count() == 2 && joker_count == 2
            {
                tier_value = 3;
            } else {
                tier_value = 2;
            }
        } else {
            tier_value = 1;
        }

        self.joker_hand_score = hand_value;
        self.joker_tier_score = tier_value;
    }
}
