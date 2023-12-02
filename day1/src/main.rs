use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut calibration_values_sum = 0;
    let part = 2;
    // Read the file and iterate over every line
    if let Ok(lines) = read_lines("src/input.txt") {
        for line in lines {
            if let Ok(mut ip) = line {
                if part == 1 {
                    // Filter out any non numerical characters
                    let re = Regex::new(r"[A-Za-z]").unwrap();
                    let filtered_string = String::from(re.replace_all(&ip, ""));
                    let first_char = filtered_string.chars().nth(0).unwrap();
                    let last_char = filtered_string.chars().last().unwrap();
                    let calibration_value: i32 =
                        format!("{}{}", first_char, last_char).parse().unwrap();
                    calibration_values_sum += calibration_value;
                } else if part == 2 {
                    // Those dang elfs making things difficult!
                    // Declare an array of the possible spelled out numbers
                    let spelled_numbers = [
                        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                    ];

                    // either we have junk or a spelled out number. Grab either the spelled out number or the numerical value, whichever comes first.
                    let first_integer_index = match ip.chars().position(|x| x.is_numeric()) {
                        None => -1,
                        i => i.unwrap() as i32,
                    };

                    let last_integer_index = match ip.chars().rev().position(|x| x.is_numeric()) {
                        None => -1,
                        i => ((ip.len() as i32 - 1) - i.unwrap() as i32) as i32,
                    };

                    let mut first_string_index = -1;
                    let mut first_matched_spelled_number = "";
                    let mut last_string_index = -1;
                    let mut last_matched_spelled_number = "";

                    for spelled_number in spelled_numbers {
                        let first_match_index = match ip.find(spelled_number) {
                            None => -1,
                            i => i.unwrap() as i32,
                        };

                        let last_match_index = match ip.rfind(spelled_number) {
                            None => -1,
                            i => i.unwrap() as i32,
                        };

                        if first_match_index >= 0 {
                            if first_match_index < first_string_index
                                || first_string_index == -1 as i32
                            {
                                first_matched_spelled_number = spelled_number;
                                first_string_index = first_match_index;
                            }
                        }

                        if last_match_index >= 0 {
                            if last_match_index > last_string_index
                                || last_string_index == -1 as i32
                            {
                                last_matched_spelled_number = spelled_number;
                                last_string_index = last_match_index;
                            }
                        }
                    }

                    let first_char: char;
                    let last_char: char;

                    if first_integer_index != -1
                        && (first_string_index == -1 || first_integer_index < first_string_index)
                    {
                        first_char = ip.chars().nth(first_integer_index as usize).unwrap();
                    } else {
                        first_char = match first_matched_spelled_number {
                            "one" => '1',
                            "two" => '2',
                            "three" => '3',
                            "four" => '4',
                            "five" => '5',
                            "six" => '6',
                            "seven" => '7',
                            "eight" => '8',
                            "nine" => '9',
                            _ => panic!(),
                        };
                    }

                    if last_integer_index > last_string_index {
                        last_char = ip.chars().nth(last_integer_index as usize).unwrap();
                    } else {
                        last_char = match last_matched_spelled_number {
                            "one" => '1',
                            "two" => '2',
                            "three" => '3',
                            "four" => '4',
                            "five" => '5',
                            "six" => '6',
                            "seven" => '7',
                            "eight" => '8',
                            "nine" => '9',
                            _ => panic!(),
                        };
                    }

                    let calibration_value: i32 =
                        format!("{}{}", first_char, last_char).parse().unwrap();
                    calibration_values_sum += calibration_value;
                }
            }
        }
    }
    println!("The answer is {calibration_values_sum}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
