pub fn task_one(content: String) -> String {
    println!("Day 1! task 1");
    const RADIX: u32 = 10;
    let lines = content.lines();

    let mut total_number = 0;

    for line in lines {
        let number_characters: Vec<char> = line.chars().filter(|&c| c.is_digit(RADIX)).collect();
        let string_number = number_characters.first().unwrap().to_string() + &*number_characters.last().unwrap().to_string();
        let number = string_number.parse::<i32>().unwrap();
        total_number = total_number + number;
    }


    return total_number.to_string();
}

pub fn task_two(content: String) -> String {
    println!("Day 1! task 2");
    let mut total_number = 0;
    let lines = content.lines();
    let numbers_as_text = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let numbers_list = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    for line in lines {
        let first_number = find_first(numbers_as_text, numbers_list, line);
        let last_number = find_last(numbers_as_text, numbers_list, line);
        let string_number = format!("{}{}", first_number, last_number);
        let number = string_number.parse::<i32>().unwrap();

        total_number = total_number + number;
    }


    return total_number.to_string();
}


fn find_first(numbers_as_text: [&str; 9], numbers_list: [&str; 9], line: &str) -> i32 {
    let mut lowest_iteration = line.chars().count();
    let mut current_used_value = "";


    for current_number in [numbers_as_text, numbers_list].concat() {
        let first_index = line.find(current_number);
        match first_index {
            Some(start) => {
                if start <= lowest_iteration {
                    lowest_iteration = start;
                    current_used_value = current_number;
                }
            }
            _ => {}
        }
    }

    return convert_to_number(current_used_value);
}

fn find_last(numbers_as_text: [&str; 9], numbers_list: [&str; 9], line: &str) -> i32 {
    let mut highest_iteration = 0;
    let mut current_used_value = "";


    for current_number in [numbers_as_text, numbers_list].concat() {
        let first_index = line.rfind(current_number);
        match first_index {
            Some(start) => {
                if start >= highest_iteration {
                    highest_iteration = start;
                    current_used_value = current_number;
                }
            }
            _ => {}
        }
    }

    return convert_to_number(current_used_value);
}

fn convert_to_number(number: &str) -> i32{
    return match number {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven"=> 7,
        "eight"=>8,
        "nine"=>9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7"=> 7,
        "8"=>8,
        "9"=>9,
        _ => {
            println!("ERROR {}", number);
            return 0;

        }
    }
}