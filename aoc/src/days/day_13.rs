use crate::problem::Problem;
use std::cmp::Ordering;
use regex::Regex;

pub struct DayThirteen;

enum Comparison {
    Correct, 
    Incorrect,
    Undetermined
}

fn compare_line_recursive(left: &mut String, right: &mut String) -> Comparison {
    loop {
        if left == "" && right == "" {
            return Comparison::Undetermined
        } else if left == "" {
            return Comparison::Correct
        } else if right == "" {
            return Comparison::Incorrect
        }

        let comparison;

        if left.starts_with("]") && right.starts_with("]") {
            lazy_static! {
                static ref REGEX_EOL: Regex = Regex::new(r"^\](,)?(?P<remaining>[0-9\s\]\[,]*)$").unwrap();
            }
            *left = String::from(REGEX_EOL.captures(left).unwrap().name("remaining").unwrap().as_str());
            *right = String::from(REGEX_EOL.captures(right).unwrap().name("remaining").unwrap().as_str());
            comparison = Comparison::Undetermined;
        } else if left.starts_with("]") && ! right.starts_with("]") {
            comparison = Comparison::Correct;
        } else if ! left.starts_with("]") && right.starts_with("]") {
            comparison = Comparison::Incorrect;
        } else if left.starts_with("[") && right.starts_with("[") {
            left.drain(0..1);
            right.drain(0..1);
            comparison = compare_line_recursive(left, right);
        } else if left.starts_with("[") && ! right.starts_with("[") {
            left.drain(0..1);
            let mut tmp_string = format!("{}]", right.chars().next().unwrap());
            comparison = compare_line_recursive(left, &mut tmp_string);
            right.drain(0..1);
        } else if ! left.starts_with("[") && right.starts_with("[") {
            right.drain(0..1);
            let mut tmp_string = format!("{}]", left.chars().next().unwrap());
            comparison = compare_line_recursive(&mut tmp_string, right);
            left.drain(0..1);
        } else {
            lazy_static! {
                static ref REGEX_VAL: Regex = Regex::new(r"^(?P<num>[0-9]+)(,)?(?P<remaining>[0-9\s\]\[,]*)$").unwrap();
            }
            
            let left_val: u32 = REGEX_VAL.captures(left).unwrap().name("num").unwrap().as_str().parse().unwrap();
            *left = String::from(REGEX_VAL.captures(left).unwrap().name("remaining").unwrap().as_str());
            let right_val: u32 = REGEX_VAL.captures(right).unwrap().name("num").unwrap().as_str().parse().unwrap();
            *right = String::from(REGEX_VAL.captures(right).unwrap().name("remaining").unwrap().as_str());
            
            comparison = match left_val.cmp(&right_val) {
                Ordering::Less => Comparison::Correct,
                Ordering::Equal => Comparison::Undetermined,
                Ordering::Greater => Comparison::Incorrect,
            };
        }
        
        match comparison {
            Comparison::Correct => {return Comparison::Correct},
            Comparison::Incorrect => {return Comparison::Incorrect},
            Comparison::Undetermined => {continue;},
        }
    }
}


fn compare_line(line1: &str, line2: &str) -> Comparison {
    let mut left = String::from(line1);
    let mut right = String::from(line2);
    
    compare_line_recursive(&mut left, &mut right)
}


fn compare(input: &str) -> u32 {
    let mut score: u32 = 0;
    for (i, pair) in input.split("\n\n").enumerate() {
        let num = i + 1;
        let lines: Vec<&str> = pair.lines().collect();
        match compare_line(lines[0], lines[1]) {
            Comparison::Incorrect => {println!("{num} incorrect"); continue},
            Comparison::Correct => {println!("{num} correct"); score += i as u32 + 1;},
            Comparison::Undetermined => panic!("Undetermined score for pair {i}"),
        }
    }
    score
}

fn divider_packet_index(divider: &str, input: &str) -> u32 {
    let mut less_than_count = 0;

    for line in input.lines() {
        if line == "" {
            continue;
        }
        match compare_line(line, divider) {
            Comparison::Correct => {less_than_count += 1},
            Comparison::Incorrect => {continue},
            Comparison::Undetermined => {continue},
        }
    }

    dbg!(less_than_count);

    less_than_count + 1
}

fn divider_packets(input: &str) -> u32 {
    let divider_1 = "[[2]]";
    let divider_2 = "[[6]]";
    
    let divider_1_index = divider_packet_index(divider_1, input);
    let divider_2_index = divider_packet_index(divider_2, input);

    // + 1 divider 2 as it is also greater than divider 1
    return divider_1_index * (divider_2_index + 1); //@@@ divider 2 195 and should be 193
}


impl Problem for DayThirteen {
    fn part_one(&self, input: &str) -> String {
        let score = compare(input);
        format!("Score: {score}")
    }

    fn part_two(&self, input: &str) -> String {
        let score = divider_packets(input);
        format!("Score: {score}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_packet_cmp_p1() {
        let output = DayThirteen{}.part_one(&INPUT);
        assert_eq!(output, "Score: 13")
    }

    #[test]
    fn test_packet_cmp_p2() {
        let output = DayThirteen{}.part_two(&INPUT);
        assert_eq!(output, "Score: 140")
    }
}
