use crate::problem::Problem;

pub struct DayOne {}

fn parse(input: &str) -> Vec<i32> {    
    // Consumes the iterator, returns an (Optional) String
    let mut sum = 0;
    let mut elf_scores = Vec::new();
    for line in input.split("\n") {
        if line == "" {
            elf_scores.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    elf_scores
}


impl Problem for DayOne {
    fn part_one(&self, input: &str) -> String {
        let elf_scores = parse(input);
        let max_value = elf_scores.iter().max();
        match max_value {
            Some(max) => format!( "Maximum elf score: {}", max ),
            None      => format!( "No elf totals found" ),
        }
    }

    fn part_two(&self, input: &str) -> String {
        let mut elf_scores = parse(input);
        // Part 2
        elf_scores.sort();
        elf_scores.reverse();
        elf_scores.truncate(3);
        let top_3_total: i32 = elf_scores.iter().sum();
        format!("Sum of top 3 elf scores: {top_3_total}")
    }
}

#[cfg(test)]
mod tests {
    //@@@ DGM add some tests
}