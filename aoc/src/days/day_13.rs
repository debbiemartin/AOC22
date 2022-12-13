// [1,1,3,1,1]
// [1,1,5,1,1] // correct
// 
// [[1],[2,3,4]]
// [[1],4] // correct
// 
// [9]
// [[8,7,6]] // incorrect
// 
// [[4,4],4,4]
// [[4,4],4,4,4] // correct
// 
// [7,7,7,7]
// [7,7,7] // incorrect
// 
// []
// [3] // correct
// 
// [[[]]]
// [[]] // incorrect
// 
// [1,[2,[3,[4,[5,6,7]]]],8,9]
// [1,[2,[3,[4,[5,6,0]]]],8,9] // incorrect

use std::cmp::Ordering;
use regex::Regex;

enum Comparison {
    Correct, 
    Incorrect,
    Undetermined
}

fn compare_recursive(left: &mut String, right: &mut String) -> Comparison {
    let mut comparison = Comparison::Undetermined;
    let mut tmp_string = String::new();

    loop {
        if left.starts_with("[") && right.starts_with("[") {
            *left = String::from(&left[1..]);
            *right = String::from(&right[1..]);
            comparison = compare_recursive(left, right);
        } else if left.starts_with("[") && ! right.starts_with("[") {
            left.drain(0..1);
            tmp_string = format!("{}]", right.as_bytes()[0]);
            comparison = compare_recursive(left, &mut tmp_string);
            right.drain(0..1);
        } else if ! left.starts_with("[") && right.starts_with("[") {
            right.drain(0..1);
            tmp_string = format!("{}]", left.as_bytes()[0]);
            comparison = compare_recursive(&mut tmp_string, right);
            left.drain(0..1);
        } else if left.starts_with("]") && right.starts_with("]") {
            let RE: Regex = Regex::new(r"^\](,)?(?P<remaining>[0-9\s\]\[,]*)$").unwrap();
            *left = String::from(RE.captures(left).unwrap().name("remaining").unwrap().as_str());
            *right = String::from(RE.captures(right).unwrap().name("remaining").unwrap().as_str());
            comparison = Comparison::Undetermined
        } else if left.starts_with("]") && ! right.starts_with("]") {
            comparison = Comparison::Correct;
        } else if ! left.starts_with("]") && right.starts_with("]") {
            comparison = Comparison::Incorrect;
        } else {
            let RE: Regex = Regex::new(r"^(?P<num>[0-9]+)(,)?(?P<remaining>[0-9\s\]\[,]*)$").unwrap();
            
            let left_val: u32 = RE.captures(left).unwrap().name("num").unwrap().as_str().parse().unwrap();
            *left = String::from(RE.captures(left).unwrap().name("remaining").unwrap().as_str());
            let right_val: u32 = RE.captures(right).unwrap().name("num").unwrap().as_str().parse().unwrap();
            *right = String::from(RE.captures(right).unwrap().name("remaining").unwrap().as_str());
            
            comparison = match left_val.cmp(&right_val) {
                Ordering::Less => Comparison::Correct,
                Ordering::Equal => Comparison::Undetermined,
                Ordering::Greater => Comparison::Incorrect,
            }
        }
        
        match comparison {
            Comparison::Correct => {return Comparison::Correct},
            Comparison::Incorrect => {return Comparison::Incorrect},
            Comparison::Undetermined => {continue;},
        }
    }
}


fn compare(line1: &str, line2: &str) -> Comparison {
    let mut left = String::from(line1);
    let mut right = String::from(line2);
    
    compare_recursive(&mut left, &mut right)
}


fn main() {
    let input = "[1,1,3,1,1]
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
    let mut score = 0;
    for (i, pair) in input.split("\n\n").enumerate() {
        let num = i + 1;
        let lines: Vec<&str> = pair.lines().collect();
        match compare(lines[0], lines[1]) {
            Comparison::Incorrect => {println!("{num} incorrect"); continue},
            Comparison::Correct => {println!("{num} correct"); score += i + 1;},
            Comparison::Undetermined => panic!("Undetermined score for pair {i}"),
        }
    }
    
    println!("Final score: {score}")
}
