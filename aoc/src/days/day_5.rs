use crate::problem::Problem;
use regex::Regex;

pub struct DayFive {}

enum Ordering {
    Lifo, 
    Fifo
}

struct Instruction {
    count: usize, 
    from: usize,
    to: usize,
}

impl Instruction {
    pub fn new(instruction: &str) -> Instruction {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\s*move\s(?P<count>[0-9]+)\sfrom\s(?P<from>[0-9]+)\sto\s(?P<to>[0-9]+)\s*$").unwrap();
        }
    
        let cap = RE.captures(instruction.trim()).unwrap();
        let count: usize = cap.name("count").unwrap().as_str().parse::<usize>().unwrap();
        let from: usize = cap.name("from").unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to: usize = cap.name("to").unwrap().as_str().parse::<usize>().unwrap() - 1;
    
        Instruction { count, from, to }
    }
}

struct Crates {
    crates: Vec<Box<Vec<char>>>,
    ordering: Ordering,
}

impl Crates {
    pub fn new(input: &str, ordering: Ordering) -> Crates {
        // rsplit reverses to traverse up the stack 
        let lines: Vec<&str> = input.rsplit("\n").collect();

        // first line example: " 1   2   3 "
        let count = lines[0].trim().split("   ").collect::<Vec<&str>>().len();
        let mut crates = Vec::new();
        for _ in 0..count {
            crates.push(Box::new(Vec::new()))
        }

        for i in 1..lines.len() {
            let line: Vec<char> = lines[i].chars().collect();
            // line example: "[Z] [M] [P]"
            // Note any combination of indices could be absent 
            for j in 0..count {
                let character = line[1 + 4 * j];
                if character != ' ' {
                    crates[j].push(character);
                } 
            }
        }
        
        Crates { crates, ordering }
    }

    fn pop(&mut self, crate_index: usize) -> char {
        self.crates[crate_index].pop().unwrap()
    }
    
    fn push(&mut self, crate_index: usize, element: char) {
        self.crates[crate_index].push(element)
    }

    fn do_move_lifo(&mut self, instruction: Instruction) {
        for _ in 0..instruction.count {
            let move_element = self.pop(instruction.from);
            self.push(instruction.to, move_element);
        }
    }

    fn do_move_fifo(&mut self, instruction: Instruction) {
        let mut tmp_vec = Vec::new();
        for _ in 0..instruction.count {
            tmp_vec.push(self.pop(instruction.from));
        }

        for _ in 0..instruction.count {
            self.push(instruction.to, tmp_vec.pop().unwrap());
        }
    }
    
    // This would be better being polymorphic but Rust doesn't have inheritance.. 
    pub fn do_move(&mut self, instruction: Instruction) {
        match self.ordering {
            Ordering::Lifo => self.do_move_lifo(instruction),
            Ordering::Fifo => self.do_move_fifo(instruction)
        }
    }

    pub fn get_final_str(&self) -> String {
        let mut result = String::new();
        for c in &self.crates {
            result.push(*c.last().unwrap());
        }
        result
    }
}

fn get_final_crates(input: &str, ordering: Ordering) -> String {
    let crates_instructions: Vec<&str> = input.split("\n\n").collect();
    let mut crates = Crates::new(crates_instructions[0], ordering);

    for instruction_str in crates_instructions[1].split("\n") {
        let instruction = Instruction::new(instruction_str);
        crates.do_move(instruction)
    }
    
    crates.get_final_str()
}


impl Problem for DayFive {

    fn part_one(&self, input: &str) -> String {
        let final_crates = get_final_crates(input, Ordering::Lifo);
        format!("Top of the crates at finish: {final_crates}")
    }

    fn part_two(&self, input: &str) -> String {
        let final_crates = get_final_crates(input, Ordering::Fifo);
        format!("Top of the crates at finish: {final_crates}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_initial_crates() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3";
        let crates = Crates::new(&input, Ordering::Lifo);
        assert_eq!(crates.crates.len(), 3);
        assert_eq!(crates.crates[0].len(), 2);
        assert_eq!(crates.crates[0][0], 'Z');
        assert_eq!(crates.crates[0][1], 'N');
        assert_eq!(crates.crates[1].len(), 3);
        assert_eq!(crates.crates[1][0], 'M');
        assert_eq!(crates.crates[1][1], 'C');
        assert_eq!(crates.crates[1][2], 'D');
        assert_eq!(crates.crates[2].len(), 1);
        assert_eq!(crates.crates[2][0], 'P');
    }


    #[test]
    fn test_parse_instruction() {
        let instruction = "    move 1 from 2 to 3";
        let instruction = Instruction::new(&instruction);
        assert_eq!(instruction.count, 1);
        assert_eq!(instruction.from, 1);
        assert_eq!(instruction.to, 2);
    }

    #[test]
    fn test_do_move_lifo() {
        let mut crates = Crates::new("[C]    
[B]    
[A] [D]
 1   2 ",
        Ordering::Lifo);
        
        crates.do_move(Instruction::new("move 2 from 1 to 2"));
        assert_eq!(crates.crates.len(), 2);
        assert_eq!(crates.crates[0].len(), 1);
        assert_eq!(crates.crates[0][0], 'A');
        assert_eq!(crates.crates[1].len(), 3);
        assert_eq!(crates.crates[1][0], 'D');
        assert_eq!(crates.crates[1][1], 'C');
        assert_eq!(crates.crates[1][2], 'B');
    }

    #[test]
    fn test_do_move_fifo() {
        let mut crates = Crates::new("[C]    
[B]    
[A] [D]
 1   2 ",
        Ordering::Fifo);
        
        crates.do_move(Instruction::new("move 2 from 1 to 2"));
        assert_eq!(crates.crates.len(), 2);
        assert_eq!(crates.crates[0].len(), 1);
        assert_eq!(crates.crates[0][0], 'A');
        assert_eq!(crates.crates[1].len(), 3);
        assert_eq!(crates.crates[1][0], 'D');
        assert_eq!(crates.crates[1][1], 'B');
        assert_eq!(crates.crates[1][2], 'C');
    }

    #[test]
    fn test_find_final_crates_p1() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = DayFive{}.part_one(&input);
        assert_eq!(result, "Top of the crates at finish: CMZ");
    }

    #[test]
    fn test_find_final_crates_p2() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let result = DayFive{}.part_two(&input);
        assert_eq!(result, "Top of the crates at finish: MCD");
    }
}