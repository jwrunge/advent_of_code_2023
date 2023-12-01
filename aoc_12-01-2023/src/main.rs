use std::{fs};

fn get_calibration(filename: &str) -> i32 {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n");
    
    let mut nums: Vec<i32> = Vec::new();
    for line in lines {
        let first_num = first_number(line);
        let last_num = last_number(line);
        // let combo_str = String::from(first_num + last_num);
        // nums.push(combo_str.parse::<i32>().expect("Combo string should parse to i32"));
    }

    let mut sum = 0;
    // for num in nums {
    //     sum += num;
    // }

    sum
}

fn first_number(input: &str) -> Option<char> {
    for c in input.chars() {
        if c.is_digit(10) {
            return Some(c);
        }
    }

    None
}

fn last_number(input: &str) -> Option<char> {
    for c in input.chars().rev() {
        if c.is_digit(10) {
            return Some(c);
        }
    }

    None
}

fn main() {
    
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
}