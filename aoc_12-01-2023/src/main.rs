use std::{fs, collections::HashMap};

fn get_calibration(filename: &str) -> i32 {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n");
    
    let mut nums: Vec<i32> = Vec::new();
    for line in lines {
        let first_num = first_number(line);
        let last_num = last_number(line);
        let combo_str = format!("{}{}", first_num.unwrap(), last_num.unwrap());
        nums.push(combo_str.parse::<i32>().expect("Combo string should parse to i32"));
    }

    let mut sum = 0;
    for num in nums {
        sum += num;
    }

    sum
}

fn first_number(input: &str) -> Option<char> {
    let replaced = check_for_word(input);
    for c in replaced.chars() {
        if c.is_digit(10) {
            return Some(c);
        }
    }

    None
}

fn last_number(input: &str) -> Option<char> {
    let replaced = check_for_word(input);
    for c in replaced.chars().rev() {
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

    let mut replaced = input.to_string();
    for word in words.keys() {
        if input.contains(word) {
            let replacement = format!("{}{}", words.get(word).unwrap(), word);
            replaced = replaced.replace(word, replacement.as_str());
        }
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
        assert_eq!(first_number(&"ab12cd34ef").unwrap(), '1');
    }

    #[test]
    fn test_last_num() {
        assert_eq!(last_number(&"ab12cd34ef").unwrap(), '4');
    }

    #[test]
    fn test_check_for_word() {
        assert_eq!(check_for_word(&"jjhxddmg5mqxqbgfivextlcpnvtwothreetwonerzk"), "jjhxddmg5mqxqbg5fivextlcpnv2two3three2tw1onerzk");
    }
}