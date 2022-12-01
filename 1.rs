use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    write_to_file(); // temp - replace this with preconstructed file 
    
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut sum = 0;
        let mut elf_scores = Vec::new();
        for line in lines {
            if let Ok(l) = line {
                if l == "" {
                    elf_scores.push(sum);
                    sum = 0;
                } else {
                    sum += l.parse::<i32>().unwrap();
                }
            }
        }
        
        // Part 1
        let max_value = elf_scores.iter().max();
        match max_value {
            Some(max) => println!( "Maximum elf score: {}", max ),
            None      => println!( "No elf totals found" ),
        }

        // Part 2
        elf_scores.sort();
        elf_scores.reverse();
        elf_scores.truncate(3);
        let top_3_total: i32 = elf_scores.iter().sum();
        println!("Sum of top 3 elf scores: {top_3_total}")
    }
}
