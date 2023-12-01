use std::fs;

// fn first_number(input: &str) -> i32 {

// }

fn load_input_from_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("File contents: {}", contents);
    "OK".to_string()
}

fn main() {
    load_input_from_file("coords.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_file() {
        assert_eq!(load_input_from_file("coords.txt"), "OK");
    }
}