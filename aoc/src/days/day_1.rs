use crate::problem::Problem;

pub struct DayOne {}

fn parse(input: &str) -> Vec<i32> {
    let mut sum = 0;
    let mut elf_scores = Vec::new();
    for line in input.split("\n") {
        if line.trim() == "" {
            elf_scores.push(sum);
            sum = 0;
        } else {
            sum += line.trim().parse::<i32>().unwrap();
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
    use super::*;

    #[test]
    fn test_d1_p1() {
        let input = "1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000
        ";
        let result: String = DayOne{}.part_one(&input);
        assert_eq!(result, "Maximum elf score: 24000");
    }

    #[test]
    fn test_d1_p2() {
        let input = "1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000
        ";
        let result: String = DayOne{}.part_two(&input);
        assert_eq!(result, "Sum of top 3 elf scores: 45000");
    }
}