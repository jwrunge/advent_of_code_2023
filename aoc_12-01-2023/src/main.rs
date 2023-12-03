use lazy_static::lazy_static;
use std::{fs, collections::HashMap};

lazy_static! {
    static ref WORDS: HashMap<String, i32> = HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);
}

fn get_calibration(filename: &str) -> i32 {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n");
    
    let mut sum: i32 = 0;
    for line in lines {
        sum += number_from_line(line);
    }

    sum
}

fn number_from_line(line: &str) -> i32 {
    let first_num = first_or_last_number(line, false);
    let last_num = first_or_last_number(line, true);
    let combo_str = format!("{}{}", first_num, last_num);
    let as_num = combo_str.parse::<i32>().expect("Combo string should parse to i32");
    as_num
}

fn first_or_last_number(input: &str, reverse: bool) -> i32 {
    let mut i = 0;

    match reverse {
        true => {
            for c in input.chars().rev() {
                if c.is_digit(10) {
                    return c.to_digit(10).unwrap() as i32;
                }
        
                match check_for_word(input, input.len() - i - 1) {
                    Some(digit) => { return digit; },
                    None => { i += 1; }
                }
            }
        }
        false => {
            for c in input.chars() {
                if c.is_digit(10) {
                    return c.to_digit(10).unwrap() as i32;
                }
        
                match check_for_word(input, i) {
                    Some(digit) => { return digit; },
                    None => { i += 1; }
                }
            }
        }
    }

    0   //Default to return 0, because why not?
}

fn check_for_word(input: &str, i: usize) -> Option<i32> {
    for word in WORDS.keys() {
        let substr = match input.get(i..i+word.len()).ok_or("Out of bounds") {
            Ok(str)=> str,
            _ => {continue;}
        };

        if substr == word {
            return WORDS.get(word).copied();
        }
    }

    None
}

fn main() {
    for _ in 1..1000 {
        let coords = get_calibration("coords.txt");
        println!("Calibration: {}", coords);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_num() {
        assert_eq!(first_or_last_number(&"ab12cd34ef", false), 1);
    }

    #[test]
    fn test_last_num() {
        assert_eq!(first_or_last_number(&"ab12cd34ef", true), 4);
    }

    #[test]
    fn test_check_for_word_nums() {
        assert_eq!(number_from_line("two1nine"), 29);
        assert_eq!(number_from_line("eightwothree"), 83);
        assert_eq!(number_from_line("abcone2threexyz"), 13);
        assert_eq!(number_from_line("xtwone3four"), 24);
        assert_eq!(number_from_line("4nineeightseven2"), 42);
        assert_eq!(number_from_line("zoneight234"), 14);
        assert_eq!(number_from_line("7pqrstsixteen"), 76);
    }
}
