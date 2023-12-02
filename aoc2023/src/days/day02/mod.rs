#[derive(Clone, Copy)]
struct Game {
    identifier: i32,
    blue: i32,
    red: i32,
    green: i32,
}

pub fn task_one(content: String) -> String {
    println!("Day 2! task 1");
    let lines = content.lines();

    let mut all_games: Vec<Game> = Vec::new();
    for line in lines {
        let game = parse_games(line);

        if validate_game_against_bag(game) {
            println!("ADDED!");
            all_games.push(game)
        }
    }

    let total_number: i32 = all_games.iter().map(|g| g.identifier).sum();
    return total_number.to_string();
}


pub fn task_two(content: String) -> String {
    println!("Day 2! task 2");
    let mut total_number = 0;
    let mut all_games: Vec<Game> = Vec::new();
    let lines = content.lines();
    for line in lines {
        let game = parse_games(line);
        all_games.push(game)
    }

    let total_number: i32 = all_games.iter().map(|g| g.blue * g.red * g.green).sum();
    return total_number.to_string();
}


fn parse_games(line: &str) -> Game {
    let mut identifier = 0;
    let mut highest_blue = 0;
    let mut highest_red = 0;
    let mut highest_green = 0;
    let line_parts = line.split_once(':');
    match line_parts {
        Some((game_identifier, games)) => {
            identifier = cleanup_value(game_identifier, "Game ");
            let individual_games: Vec<&str> = games.split(';').collect();
            for individual_game in individual_games {
                let individual_plays: Vec<&str> = individual_game.split(',').collect();
                for play in individual_plays {
                    if play.contains("blue") {
                        let current_blue = cleanup_value(play, " blue");
                        if current_blue > highest_blue {
                            highest_blue = current_blue;
                        }
                    } else if play.contains("red") {
                        let current_red = cleanup_value(play, " red");
                        if current_red > highest_red {
                            highest_red = current_red;
                        }
                    } else if play.contains("green") {
                        let current_green = cleanup_value(play, " green");
                        if current_green > highest_green {
                            highest_green = current_green;
                        }
                    }
                }
            }
        }
        _ => {}
    }


    return Game {
        identifier,
        red: highest_red,
        blue: highest_blue,
        green: highest_green,
    };
}

fn cleanup_value(game_value: &str, replace_value: &str) -> i32 {
    let string_value = game_value.replace(replace_value, "").trim().to_string();
    return string_value.parse::<i32>().unwrap();
}

fn validate_game_against_bag(game: Game) -> bool {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;


    if game.green > green_cubes {
        return false;
    }
    if game.blue > blue_cubes {
        return false;
    }
    if (game.red > red_cubes) {
        return false;
    }

    return true;
}