#[derive(Debug, Clone)]
struct Paper {
    times: Vec<u64>,
    distances: Vec<u64>,
}


pub fn task_one(content: String) -> String {
    println!("Day 6! task 1");
    let paper = parse_file_to_paper(content);

    let beat_times = calculate_beat_times(paper);

    return beat_times.to_string();
}


pub fn task_two(content: String) -> String {
    println!("Day 6! task 2");


    let paper_task_2 = parse_file_to_paper_task_two(content);

    let beat_times = calculate_beat_times(paper_task_2);

    return beat_times.to_string();
}

fn calculate_beat_times(paper: Paper) -> u64 {
    let mut total_number = 1;
    for (index, time) in paper.times.iter().enumerate() {
        let mut num: u64 = 0;
        let best_distance = paper.distances[index];
        for i in 0..=*time {
            if i * (time - i) > best_distance {
                num += 1;
            }
        }
        total_number *= num;
    }

    return total_number;
}


fn parse_file_to_paper(content: String) -> Paper {
    let mut lines = content.lines();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .trim_start_matches("Time:")
        .trim()
        .split_whitespace()
        .map(|t| t.parse::<u64>().unwrap())
        .collect();

    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .trim_start_matches("Distance:")
        .trim()
        .split_whitespace()
        .map(|d| d.parse::<u64>().unwrap())
        .collect();

    return Paper {
        times,
        distances,
    };
}

fn parse_file_to_paper_task_two(content: String) -> Paper {
    let mut lines = content.lines();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .trim_start_matches("Time:")
        .trim()
        .replace(" ", "")
        .trim()
        .split_whitespace() //i know this is not great, but i dont want to waste time
        .map(|t| t.parse::<u64>().unwrap())
        .collect();

    let distances: Vec<u64> = lines
        .next()
        .unwrap()
        .trim_start_matches("Distance:")
        .trim()
        .replace(" ", "")
        .trim()
        .split_whitespace() //i know this is not great, but i dont want to waste time
        .map(|d| d.parse::<u64>().unwrap())
        .collect();

    return Paper {
        times,
        distances,
    };
}