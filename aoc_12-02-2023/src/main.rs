use std::fs;

fn games_possible(filename: &str) -> (i32, i32) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n");
    
    let mut sum_ids: i32 = 0;
    let mut sum_powers: i32 = 0;
    for line in lines {
        let mut line_iter = line.split(":");
        let game_id = line_iter.next().unwrap().replace("Game ", "").parse::<i32>().unwrap();

        let (possible, power) = game_is_possible(line_iter.next().unwrap());
        if possible {
            sum_ids += game_id;

        }
        sum_powers += power;
    }

    (sum_ids, sum_powers)
}

fn game_is_possible(game: &str) -> (bool, i32) {
    let mut possible = true;

    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;
    
    let mut min_red = -1;
    let mut min_green = -1;
    let mut min_blue = -1;

    let rounds = game.split(";");
    for round in rounds {
        let (red, green, blue) = get_round_results(round);

        match red {
            Some(x) => {
                if x > MAX_RED {
                    possible = false;
                }
                if min_red < x || min_red == -1 {
                    min_red = x
                }
            },
            None => (),
        }
        match green {
            Some(x) => {
                if x > MAX_GREEN {
                    possible = false;
                }
                if min_green < x || min_green == -1 {
                    min_green = x
                }
            },
            None => (),
        }
        match blue {
            Some(x) => {
                if x > MAX_BLUE {
                    possible = false;
                }
                if min_blue < x || min_blue == -1 {
                    min_blue = x
                }
            },
            None => (),
        }
    }

    if min_red == -1 {
        min_red = 0;
    }
    if min_green == -1 {
        min_green = 0;
    }
    if min_blue == -1 {
        min_blue = 0;
    }

    let total_power = min_red * min_green * min_blue;

    (possible, total_power)
}

fn get_round_results(round: &str) -> (Option<i32>, Option<i32>, Option<i32>) {
    let color_strs = round.split(",");
    let mut red = None;
    let mut green = None;
    let mut blue = None;

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
                    "red" => {
                        red = Some(value);
                    },
                    "green" => {
                        green = Some(value);
                    },
                    "blue" => {
                        blue = Some(value); 
                    },
                    _ => panic!("Invalid color: {}", color),
                }
            }
        }
    }

    (red, green, blue)
}

fn main() {
    let (games_possible_ids, games_possible_powers) = games_possible("games.txt");
    println!("Games: {}; powers: {}", games_possible_ids, games_possible_powers);
}