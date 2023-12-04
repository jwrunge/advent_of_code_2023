use std::fs;

fn games_possible(filename: &str) -> i32 {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n");
    
    let mut sum: i32 = 0;
    for line in lines {
        let mut line_iter = line.split(":");
        let game_id = line_iter.next().unwrap().replace("Game ", "").parse::<i32>().unwrap();
        if game_is_possible(line_iter.next().unwrap()) {
            sum += game_id;
        }
    }

    sum
}

fn game_is_possible(game: &str) -> bool {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    let rounds = game.split(";");
    for round in rounds {
        println!("Round: {}", round);
        let (red, green, blue) = get_round_results(round);

        if red > MAX_RED || green > MAX_GREEN || blue > MAX_BLUE {
            println!("\nGame is not possible: {}", game);
            println!("RED: {}, GREEN: {}, BLUE: {}", red, green, blue);
            return false;
        }
    }

    true
}

fn get_round_results(round: &str) -> (i32, i32, i32) {
    let color_strs = round.split(",");
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for color_str in color_strs {
        for color in ["red", "green", "blue"] {
            if color_str.contains(color) {
                let digit_str = color_str.replace(color, "");

                let value = match digit_str.trim().parse::<i32>() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Invalid parse: {}, {}", color_str, &digit_str);
                        0
                    },
                };
                
                match color {
                    "red" => red = value,
                    "green" => green = value,
                    "blue" => blue = value,
                    _ => panic!("Invalid color: {}", color),
                }
            }
        }
    }

    (red, green, blue)
}

fn main() {
    let games = games_possible("games.txt");
    println!("Games: {}", games);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_first_num() {
    //     assert_eq!(first_or_last_number(&"ab12cd34ef", false), 1);
    // }
}
