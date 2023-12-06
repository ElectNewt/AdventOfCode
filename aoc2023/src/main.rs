use std::fs::File;
use std::io::{self, Read};

mod days {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
}


fn main() {
    let relative_path = "day05/sample.txt";
    match read_daily_file(relative_path) {
        Ok(content) => {
            let result01 = days::day05::task_one(content.to_string());
            println!("Task 1: {} \n", result01);
         //   let result02 = days::day05::task_two(content.to_string());
            //println!("Task 2: {} \n", result02);
            Ok(())
        }
        Err(e) => Err(e),
    }.expect("TODO: panic message");
}

fn read_daily_file(relative_path: &str) -> io::Result<String> {
    // Get the current working directory
    let current_dir = std::env::current_dir()?;

    // Construct the full path by joining the current directory, "days", and the relative path
    let file_path = current_dir.join("src").join("days").join(relative_path);

    // Open the file in read-only mode
    let mut file = File::open(&file_path)?;

    // Read the contents of the file into a String
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}
