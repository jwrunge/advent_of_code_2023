use std::{fs, collections::HashMap};

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
    let replaced: String = check_for_word(line);
    let first_num = first_or_last_number(&replaced, false);
    let last_num = first_or_last_number(&replaced, true);
    let combo_str = format!("{}{}", first_num.unwrap(), last_num.unwrap());
    let as_num = combo_str.parse::<i32>().expect("Combo string should parse to i32");
    as_num
}

fn first_or_last_number(input: &str, reverse: bool) -> Option<char> {
    let new_str = match reverse {
        true => input.chars().rev().collect::<String>(),
        false => input.to_string()
    };

    println!("New string: {}", new_str);

    for c in new_str.chars() {
        if c.is_digit(10) {
            return Some(c);
        }
    }

    None
}

fn check_for_word(input: &str) -> String {
    let words: HashMap<String, &str> = HashMap::from([
        ("one".to_string(), "1"),
        ("two".to_string(), "2"),
        ("three".to_string(), "3"),
        ("four".to_string(), "4"),
        ("five".to_string(), "5"),
        ("six".to_string(), "6"),
        ("seven".to_string(), "7"),
        ("eight".to_string(), "8"),
        ("nine".to_string(), "9"),
    ]);

    let mut replaced = "".to_string();
    let mut i = 0;
    for char in input.chars() {
        for word in words.keys() {
            let substr = match input.get(i..i+word.len()).ok_or("Out of bounds") {
                Ok(str)=> str,
                _ => {continue;}
            };

            if substr == word {
                replaced.push_str(words.get(word).unwrap());
            }
        }

        replaced.push_str(char.to_string().as_str());
        i += 1;
    }

    replaced
}

fn main() {
    let coords = get_calibration("coords.txt");
    println!("Calibration: {}", coords);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_num() {
        assert_eq!(first_or_last_number(&"ab12cd34ef", false).unwrap(), '1');
    }

    #[test]
    fn test_last_num() {
        assert_eq!(first_or_last_number(&"ab12cd34ef", true).unwrap(), '4');
    }

    #[test]
    fn test_check_for_word() {
        assert_eq!(check_for_word(&"jjhxddmg5mqxqbgfivextlcpnvtwothreetwonerzk"), "jjhxddmg5mqxqbg5fivextlcpnv2two3three2tw1onerzk");
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
