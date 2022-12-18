pub mod days;
pub mod problem;
pub mod graph;

#[macro_use]
extern crate lazy_static;

use crate::problem::Problem;
use crate::days::*;
use std::fs;
use std::env;
use std::time::Instant;


fn main() {
    let day = get_day().unwrap();

    let input_name = format!("./src/inputs/input_{}.txt", day);
    let input = read_file(input_name);
    let problem = day_to_problem(day).unwrap();
    let now = Instant::now();
    let part_1 = problem.part_one(&input);
    println!("Part 1 answer: {}", part_1);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    
    let now = Instant::now();
    let part_2 = problem.part_two(&input);
    println!("Part 2 answer: {}", part_2);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
} 

fn get_day() -> Option<usize> {
    let args: Vec<String> = env::args().collect();

    let arg = match args.get(1) {
        Some(val) => val,
        None => {
            println!("Not enough arguments provided!");
            return None;
        }
    };

    match arg.parse::<usize>() {
        Ok(val) => Some(val),
        Err(e) => {
            println!("Unable to cast argument {} to usize: {}", arg, e);
            return None;
        }
    }
}
    

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_file(filepath: String) -> String {
    println!("{filepath}");
    fs::read_to_string(filepath)
        .expect("Should have been able to read the file")
}


fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(day_1::DayOne{})),
        2 => Some(Box::new(day_2::DayTwo{})),
        3 => Some(Box::new(day_3::DayThree{})),
        4 => Some(Box::new(day_4::DayFour{})),
        5 => Some(Box::new(day_5::DayFive{})),
        6 => Some(Box::new(day_6::DaySix{})),
        7 => Some(Box::new(day_7::DaySeven{})),
        8 => Some(Box::new(day_8::DayEight{})),
        9 => Some(Box::new(day_9::DayNine{})),
        10 => Some(Box::new(day_10::DayTen{})),
        11 => Some(Box::new(day_11::DayEleven{})),
        12 => Some(Box::new(day_12::DayTwelve{})),
        13 => Some(Box::new(day_13::DayThirteen{})),
        14 => Some(Box::new(day_14::DayFourteen{})),
        15 => Some(Box::new(day_15::DayFifteen{})),
        17 => Some(Box::new(day_17::DaySeventeen{})),
        18 => Some(Box::new(day_18::DayEighteen{})),
        _ => None
    }
}