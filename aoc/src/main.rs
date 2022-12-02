pub mod days;
pub mod problem;

use crate::problem::Problem;
use crate::days::day_1::DayOne;
use crate::days::day_2::DayTwo;
use std::fs;
use std::env;

fn main() {
    let day = get_day().unwrap();

    let input_name = format!("./src/inputs/input_{}.txt", day);
    let input = read_file(input_name);
    let problem = day_to_problem(day).unwrap();
    let part_1 = problem.part_one(&input);
    println!("Part 1 answer: {}", part_1);
    
    let part_2 = problem.part_two(&input);
    println!("Part 2 answer: {}", part_2);
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
        1 => Some(Box::new(DayOne{})),
        2 => Some(Box::new(DayTwo{}))
        // ...
        _ => None
    }
}