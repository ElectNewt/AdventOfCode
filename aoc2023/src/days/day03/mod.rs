pub fn task_one(content: String) -> String {
    println!("Day 3! task 1");
    let mut total_number = 0;

    let lines = content.lines();
    let mut char_lines: Vec<Vec<char>> = vec![];
    //todo: figure out how to do this in one line
    for line in lines {
        char_lines.push(line.chars().collect());
    }

    let mut symbols_position: Vec<Vec<usize>> = Vec::new();
    for char_line in char_lines.clone() {
        let result = find_symbols_positions(char_line);
        symbols_position.push(result);
    }

    //iterate the symbol positions and do +1 and -1 around him.


    for (row_index, row) in symbols_position.iter().enumerate() {
        for &symbol_position in row {
            let adjacent_values = get_adjacent_values(row_index, symbol_position, &char_lines);
            total_number = total_number + adjacent_values;
        }
    }

    return total_number.to_string();
}

pub fn task_two(content: String) -> String {
    println!("Day 3! task 2");
    let mut total_number = 0;

    let lines = content.lines();
    let mut char_lines: Vec<Vec<char>> = vec![];
    //todo: figure out how to do this in one line
    for line in lines {
        char_lines.push(line.chars().collect());
    }

    let mut symbols_position: Vec<Vec<usize>> = Vec::new();
    for char_line in char_lines.clone() {
        let result = find_gears_positions(char_line);
        symbols_position.push(result);
    }

    //iterate the symbol positions and do +1 and -1 around him.


    for (row_index, row) in symbols_position.iter().enumerate() {
        for &symbol_position in row {
            let adjacent_values = get_gears_values(row_index, symbol_position, &char_lines);
            total_number = total_number + adjacent_values;
        }
    }

    return total_number.to_string();
}


fn get_gears_values(current_row: usize, symbol_position: usize, char_lines: &Vec<Vec<char>>) -> i32 {
    let mut total_adjacent_values = 0;
    let mut previous_row_value = 0;
    let mut next_row_value = 0;
    let mut previous_row_totals = 0;
    let mut next_row_totals = 0;
    let mut totals = 0;
    //previous row
    if current_row >= 0 {
        let r1 = get_adjacent_row_value(char_lines[current_row - 1].clone(), symbol_position, false);
        previous_row_value = r1.0;
        previous_row_totals = r1.1;
        totals += r1.1;
    }


    //Current row
    let r2 = get_adjacent_row_value(char_lines[current_row].clone(), symbol_position, false);
    let current_row_value = r2.0;
    let current_row_totals = r2.1;
    totals += r2.1;

    //next row
    if current_row <= char_lines.len() {
        let r3 = get_adjacent_row_value(char_lines[current_row + 1].clone(), symbol_position, false);
        next_row_value = r3.0;
        next_row_totals = r3.1;
        totals += r3.1;
    }

    if (totals == 2) {
        let mut first_value = 0;
        let mut last_value = 0;

        if (previous_row_totals == 2) {
            return previous_row_value;
        } else if previous_row_totals == 1 {
            first_value = previous_row_value;
        }

        if current_row_totals == 2 {
            return current_row_value;
        } else if current_row_totals == 1 {
            if first_value == 0 {
                first_value = current_row_value;
            } else {
                last_value = current_row_value;
            }
        }

        if next_row_totals == 2 {
            return next_row_value;
        } else if next_row_totals == 1 {
            last_value = next_row_value;
        }

        return first_value * last_value;
    }


    return 0;
}

fn get_adjacent_values(current_row: usize, symbol_position: usize, char_lines: &Vec<Vec<char>>) -> i32 {
    let mut total_adjacent_values = 0;
    //previous row
    if current_row >= 0 {
        let previous_row_value = get_adjacent_row_value(char_lines[current_row - 1].clone(), symbol_position, true);
        total_adjacent_values = total_adjacent_values + previous_row_value.0;
    }

    //Current row
    let current_row_value = get_adjacent_row_value(char_lines[current_row].clone(), symbol_position, true);
    total_adjacent_values = total_adjacent_values + current_row_value.0;

    //next row
    if current_row <= char_lines.len() {
        let current_row_value = get_adjacent_row_value(char_lines[current_row + 1].clone(), symbol_position, true);
        total_adjacent_values = total_adjacent_values + current_row_value.0;
    }
    return total_adjacent_values;
}

fn get_adjacent_row_value(char_line: Vec<char>, symbol_position: usize, sum: bool) -> (i32, i32) {
    //get previous
    let mut previous_result = 0;
    let mut current_result = 0;
    let mut next_result = 0;

    if symbol_position > 0 && char_line.len() >= symbol_position - 1 {
        match convert_char_to_number(symbol_position - 1, char_line.clone()) {
            Ok(result) => {
                previous_result = result;
            }
            _ => {}
        }
    }

    //get current position
    if char_line.len() >= symbol_position {
        match convert_char_to_number(symbol_position, char_line.clone()) {
            Ok(result) => {
                current_result = result;
            }
            _ => {}
        }
    }

    //get next position
    if char_line.len() >= symbol_position + 1 {
        match convert_char_to_number(symbol_position + 1, char_line.clone()) {
            Ok(result) => {
                next_result = result;
            }
            _ => {}
        }
    }

    if (current_result == 0) {
        let mut totals = 0;
        if previous_result > 0 {
            totals += 1;
        }
        if (next_result > 0) {
            totals += 1;
        }

        return if (sum) {
            (previous_result + next_result, 2)
        } else {
            if (previous_result == 0) {
                return (next_result, totals);
            } else if next_result == 0 {
                return (previous_result, totals);
            }
            return (previous_result * next_result, totals);
        };
    }

    if current_result > 0 {
        return (current_result, 1);
    }

    return (current_result, 0);
}


//I could not find a better way to do the try catch.
//this is what i did originally thinking it will check only one digit :facepalm:
/*
fn convert_char_to_number(position: usize, char_line: Vec<char>) -> Result<u32, &'static str> {
    let char_value = char_line[position];
    if char_value.is_digit(10) {
        return Ok(char_value.to_digit(10).unwrap());
    }
    return Ok(0);
}
*/

fn convert_char_to_number(position: usize, char_line: Vec<char>) -> Result<i32, &'static str> {
    let char_value = char_line[position];
    let mut start_number_index = 0;
    let mut end_number_index = 0;

    if char_value.is_digit(10) {
        for i in (0..=position).rev() {
            if char_line[i].is_digit(10) {
                start_number_index = i;
            } else {
                break;
            }
        }

        for i in (position..char_line.len()) {
            if char_line[i].is_digit(10) {
                end_number_index = i;
            } else {
                break;
            }
        }

        let string_value: String = char_line[start_number_index..=end_number_index].to_vec().iter().collect();

        if let Ok(number) = string_value.parse::<i32>() {
            return Ok(number);
        } else {
            eprintln!("ERRORRRR {}", string_value);
        }
    }
    return Ok(0);
}


fn find_symbols_positions(char_line: Vec<char>) -> Vec<usize> {
    let mut symbols_in_line: Vec<usize> = Vec::new();
    for (index, &character) in char_line.iter().enumerate() {
        if is_symbol(character) {
            symbols_in_line.push(index);
        }
    }
    return symbols_in_line;
}

fn find_gears_positions(char_line: Vec<char>) -> Vec<usize> {
    let mut symbols_in_line: Vec<usize> = Vec::new();
    for (index, &character) in char_line.iter().enumerate() {
        if is_gear(character) {
            symbols_in_line.push(index);
        }
    }
    return symbols_in_line;
}


fn is_symbol(value: char) -> bool {
    if value.is_digit(10) {
        return false;
    }

    if value == '.' {
        return false;
    }
    return true;
}

fn is_gear(value: char) -> bool {
    if value.is_digit(10) {
        return false;
    }

    if value == '.' {
        return false;
    }
    if value == '*' {
        return true;
    }

    return false;
}