use crate::solutions::*;
use std::env;
use std::fs;

mod solutions;

macro_rules! solve {
    ($day:path, $input:expr) => {{
        use $day::*;
        println!("Starting solution for day {}", DAY_NUMBER);
        println!("Part one result = {}", part_one($input));
        println!("Part two result = {}", part_two($input));
    }}
}

fn main() {

    let args: Vec<String> = env::args().collect();
    
    if (args.len() < 2)
    {
        panic!("A day to run is a required parameter");
    }

    let day_to_run = args[1].parse::<u8>();

    let day = match day_to_run {
        Ok(day) => day, 
        Err(_) => {
            panic!("Could not parse day to run");
        }
    };

    if day < 1 || day > 25 {
        panic!("Day must be a valid number from 1-25");
    }
    
    println!("HAPPY ADVENT OF CODE 2022!");
    println!("Running day #{}", day);

    let input = get_input_file("input", day);

    match day {
        1 => solve!(day1, &input),
        _ => println!("No solution found for day {}", day)
    } 
}

pub fn get_input_file(path: &str, day: u8) -> String {
    let dir = env::current_dir().unwrap();

    let filepath = dir
        .join("src")
        .join(path)
        .join(format!("day{}.txt", day));

    fs::read_to_string(filepath)
        .unwrap_or_else(|_| panic!("Could not load input file for day {}", day))
}