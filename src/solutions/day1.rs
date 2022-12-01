pub static DAY_NUMBER: u8 = 1; 

pub fn part_one(input: &str) -> i32 {
    let mut highest_calorie:i32 = 0;

    let lines: Vec<String> = input
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    
    let mut current_elf:i32 = 0;

    for calories in lines.iter(){
        if calories.is_empty() {
            if current_elf > highest_calorie
            {
                highest_calorie = current_elf;
            }
            current_elf = 0;
        }
        else {
            current_elf += calories.parse::<i32>().expect("Could not parse calories to i32");
        }
    }

    highest_calorie
}

pub fn part_two(input: &str) -> i32 {

    let lines: Vec<String> = input
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    
    let mut current_elf:i32 = 0;
    let mut top_elves = Elves {
        first : 0,
        second : 0, 
        third : 0
    };

    for calories in lines.iter(){
        if calories.is_empty() {            
            sort_elves(&mut top_elves, current_elf);
            current_elf = 0;
        }
        else {
            current_elf += calories.parse::<i32>().expect("Could not parse calories to i32");
        }
    }

    top_elves.first + top_elves.second + top_elves.third
}

struct Elves {
    first : i32,
    second : i32, 
    third : i32
}

fn sort_elves(elves : &mut Elves, current_elf : i32){
    if current_elf > elves.third {
        elves.third = current_elf; 
    }
    if elves.third > elves.second {
        let temp = elves.second;
        elves.second = elves.third;
        elves.third = temp;
    }
    if elves.second > elves.first {
        let temp = elves.first;
        elves.first = elves.second;
        elves.second = temp;
    }
}