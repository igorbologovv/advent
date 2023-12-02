use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn str_to_int(str_num: String) -> Option<i32> {
    str_num.parse::<i32>().ok()
}

use regex::Regex;

fn extract_numbers(input_string: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut current_word = String::new();

    let word_or_number_pattern =
        Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d+").unwrap();

    for c in input_string.chars() {
        current_word.push(c);

        if let Some(mat) = word_or_number_pattern.find(&current_word) {
            let matched_part = mat.as_str();

            if let Ok(num) = matched_part.parse::<i32>() {
                numbers.push(num);
            } else {
                numbers.push(word_to_number(matched_part));
            }

            current_word.clear();
            current_word.push(c);
        }
    }

    if let Ok(num) = current_word.parse::<i32>() {
        numbers.push(num);
    }

    numbers
}

fn word_to_number(word: &str) -> i32 {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}

fn main() -> io::Result<()> {
    let file = File::open("/home/turist/advent2023/task1.txt")?;
    let reader = BufReader::new(file);
    let mut array_to_sum: Vec<i32> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let numbers = extract_numbers(&content);

                    if numbers.len() == 1 {
                    // Concatenate one into 2 digits like 7 into 77
                    let two_digit_from_one: String =
                        numbers[0].to_string() + &numbers[0].to_string();

                    if let Ok(result_i32) = two_digit_from_one.parse::<i32>() {
                        array_to_sum.push(result_i32);
                    }
                    else{
                    println!("FUCK THIS SHIT");
                    }
                } else {
                    let two_digit_from_one: String =
                        numbers[0].to_string() + &numbers[numbers.len() - 1].to_string();
                    match str_to_int(two_digit_from_one) {
                        Some(number) => array_to_sum.push(number),
                        None => println!("Conversion to i32 failed"),
                    }
                }
                println!("{:?}", numbers);
            }
            Err(e) => println!("Error reading line: {}", e),
        }
    }

    let sum_of_numbers: i32 = array_to_sum.iter().sum();
    println!("Sum of numbers: {}", sum_of_numbers);

    Ok(())
}
