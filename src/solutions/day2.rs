pub static DAY_NUMBER: u8 = 2; 

pub fn part_one(input: &str) -> i32 {
    let lines: Vec<String> = input
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let mut totalScore: i32 = 0;
    for round in &lines {
        let (opp, me) = round.split_at(1);

        let mut opp_parsed = RPS::Rock;
        let mut me_parsed = RPS::Rock;

        //println!("{}{}", &opp, &me);

        if opp.trim() == "B"{
            opp_parsed = RPS::Paper;
        }
        else if opp.trim() == "C"{
            opp_parsed = RPS::Scissors;
        }

        totalScore += 1;
        if me.trim() == "Y"{
            me_parsed = RPS::Paper;
            totalScore += 1;
        }
        else if me.trim() == "Z" {
            me_parsed = RPS::Scissors;
            totalScore += 2;
        }

        //println!("{:?}{:?}", &opp_parsed, &me_parsed);
        if opp_parsed == me_parsed {
            totalScore += 3;
        }
        else {
            if opp_parsed == RPS::Rock {
                if me_parsed == RPS::Paper {
                    totalScore += 6;
                }
            } 
            else if opp_parsed == RPS::Paper {
                if me_parsed == RPS::Scissors {
                    totalScore += 6;
                }
            }
            else if opp_parsed == RPS::Scissors {
                if me_parsed == RPS::Rock {
                    totalScore += 6;
                }
            }
        }
    }
    
    totalScore
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

pub fn part_two(input: &str) -> i32 {
    let lines: Vec<String> = input
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let mut totalScore: i32 = 0;
    for round in &lines {
        let (opp, me) = round.split_at(1);

        let mut opp_parsed = RPS::Rock;
        let mut me_parsed = RPS::Rock;

        //println!("{}{}", &opp, &me);

        if opp.trim() == "B"{
            opp_parsed = RPS::Paper;
        }
        else if opp.trim() == "C"{
            opp_parsed = RPS::Scissors;
        }

        totalScore += 1;
        if me.trim() == "Y"{
            me_parsed = RPS::Paper;
            totalScore += 1;
        }
        else if me.trim() == "Z" {
            me_parsed = RPS::Scissors;
            totalScore += 2;
        }

        //println!("{:?}{:?}", &opp_parsed, &me_parsed);
        if opp_parsed == me_parsed {
            totalScore += 3;
        }
        else {
            if opp_parsed == RPS::Rock {
                if me_parsed == RPS::Paper {
                    totalScore += 6;
                }
            } 
            else if opp_parsed == RPS::Paper {
                if me_parsed == RPS::Scissors {
                    totalScore += 6;
                }
            }
            else if opp_parsed == RPS::Scissors {
                if me_parsed == RPS::Rock {
                    totalScore += 6;
                }
            }
        }
    }
    
    totalScore
}
