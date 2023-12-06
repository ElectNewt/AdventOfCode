#[derive(Debug, Clone)]
struct Map {
    source: String,
    destination: String,
    destination_range_start: i32,
    destination_range_end: i32,
    range_length: i32,
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds_to_plant: Vec<i32>,
    seed_to_soil_map: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}


pub fn task_one(content: String) -> String {
    println!("Day 5! task 1");
    let mut total_number = 0;
     map_file_to_almanac(content);

    return "this is crazy long, not event trying".to_string();
}

pub fn task_two(content: String) -> String {
    println!("Day 5! task 2");
    let mut total_number = 0;


    return "this is crazy long, not event trying".to_string();
}


fn map_file_to_almanac(content: String)  {
    let groups: Vec<_> = content.split("\n\n").collect();
    let seeds_string: Vec<_> = groups[0].split_whitespace().collect();
    let seeds: Vec<_> = seeds_string[1..seeds_string.len()].iter()
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();
    for seed in  seeds{
        println!("Seed: {}", seed)
    }




}

