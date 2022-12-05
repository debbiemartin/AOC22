use crate::problem::Problem;
use regex::Regex;

pub struct DayFive {}

fn parse_initial_stacks(stack_input: &str) -> Vec<Box<Vec<char>>> {
    // rsplit reverses to traverse up the stack 
    let lines: Vec<&str> = stack_input.rsplit("\n").collect();

    // first line example: " 1   2   3 "
    let stack_count = lines[0].trim().split("   ").collect::<Vec<&str>>().len();
    let mut stacks: Vec<Box<Vec<char>>> = Vec::new();
    for _ in 0..stack_count {
        stacks.push(Box::new(Vec::new()))
    }

    for i in 1..lines.len() {
        let line: Vec<char> = lines[i].chars().collect();
        // line example: "[Z] [M] [P]"
        // Note any combination of indices could be absent 
        for j in 0..stack_count {
            let character = line[1 + 4 * j];
            if character != ' ' {
                stacks[j].push(character);
            } 
        }
    }

    stacks
}

fn parse_instruction(instruction: &str) -> (usize, usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*move\s(?P<count>[0-9]+)\sfrom\s(?P<from>[0-9]+)\sto\s(?P<to>[0-9]+)\s*$").unwrap();
    }

    let cap = RE.captures(instruction.trim()).unwrap();

    return (
        cap.name("count").unwrap().as_str().parse().unwrap(), 
        cap.name("from").unwrap().as_str().parse().unwrap(), 
        cap.name("to").unwrap().as_str().parse().unwrap(), 
    )
}

fn do_move_lifo(stacks: &mut Vec<Box<Vec<char>>>, count: usize, from: usize, to: usize) {
    for _ in 0..count {
        let move_element = stacks[from].pop().unwrap();
        stacks[to].push(move_element)
    }
}

fn do_move_fifo(stacks: &mut Vec<Box<Vec<char>>>, count: usize, from: usize, to: usize) {
    let mut tmp_vec = Vec::new();
    for _ in 0..count {
        tmp_vec.push(stacks[from].pop().unwrap());
    }

    for _ in 0..count {
        stacks[to].push(tmp_vec.pop().unwrap());
    }
}

fn get_final_stacks<F: Fn(&mut Vec<Box<Vec<char>>>, usize, usize, usize)>(input: &str, do_move: F) -> String {
    let stacks_instructions: Vec<&str> = input.split("\n\n").collect();
    let mut stacks: Vec<Box<Vec<char>>> = parse_initial_stacks(stacks_instructions[0]);

    for instruction in stacks_instructions[1].split("\n") {
        let (count, from, to) = parse_instruction(instruction);
        do_move(&mut stacks, count, from - 1, to - 1);
    }
    
    let mut result = String::new();
    for mut stack in stacks {
        result.push(stack.pop().unwrap());
    }
    result
}


impl Problem for DayFive {
    fn part_one(&self, input: &str) -> String {
        let final_stacks = get_final_stacks(input, do_move_lifo);
        format!("Top of the stacks at finish: {final_stacks}")
    }

    fn part_two(&self, input: &str) -> String {
        let final_stacks = get_final_stacks(input, do_move_fifo);
        format!("Top of the stacks at finish: {final_stacks}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_initial_stacks() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3";
        let stacks = parse_initial_stacks(&input);
        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0].len(), 2);
        assert_eq!(stacks[0][0], 'Z');
        assert_eq!(stacks[0][1], 'N');
        assert_eq!(stacks[1].len(), 3);
        assert_eq!(stacks[1][0], 'M');
        assert_eq!(stacks[1][1], 'C');
        assert_eq!(stacks[1][2], 'D');
        assert_eq!(stacks[2].len(), 1);
        assert_eq!(stacks[2][0], 'P');
    }


    #[test]
    fn test_parse_instruction() {
        let instruction = "    move 1 from 2 to 3";
        let (count, from, to) = parse_instruction(&instruction);
        assert_eq!(count, 1);
        assert_eq!(from, 2);
        assert_eq!(to, 3);
    }

    #[test]
    fn test_do_move_lifo() {
        let mut stacks = vec![Box::new(vec!['A', 'B', 'C']), Box::new(vec!['D'])];
        do_move_lifo(&mut stacks, 2, 0, 1);
        assert_eq!(stacks.len(), 2);
        assert_eq!(stacks[0].len(), 1);
        assert_eq!(stacks[0][0], 'A');
        assert_eq!(stacks[1].len(), 3);
        assert_eq!(stacks[1][0], 'D');
        assert_eq!(stacks[1][1], 'C');
        assert_eq!(stacks[1][2], 'B');
    }

    #[test]
    fn test_do_move_fifo() {
        let mut stacks = vec![Box::new(vec!['A', 'B', 'C']), Box::new(vec!['D'])];
        do_move_fifo(&mut stacks, 2, 0, 1);
        assert_eq!(stacks.len(), 2);
        assert_eq!(stacks[0].len(), 1);
        assert_eq!(stacks[0][0], 'A');
        assert_eq!(stacks[1].len(), 3);
        assert_eq!(stacks[1][0], 'D');
        assert_eq!(stacks[1][1], 'B');
        assert_eq!(stacks[1][2], 'C');
    }

    #[test]
    fn test_find_final_stacks_p1() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = DayFive{}.part_one(&input);
        assert_eq!(result, "Top of the stacks at finish: CMZ");
    }

    #[test]
    fn test_find_final_stacks_p2() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = DayFive{}.part_two(&input);
        assert_eq!(result, "Top of the stacks at finish: MCD");
    }
}