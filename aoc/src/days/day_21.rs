use crate::Problem;
use std::collections::HashMap;
use regex::Regex;

pub struct DayTwentyOne;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}

struct Monkey {
    name: String,
    sum: Option<(String, String, Operation)>,
    number: Option<u64>
}

impl Monkey {
    fn new(input: &str) -> Monkey {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<name>[a-z]+):\s(((?P<name1>[a-z]+)\s(?P<operator>[+\-*/])\s(?P<name2>[a-z]+))|(?P<number>[0-9]+))$").unwrap();
        }
        let cap = RE.captures(input.trim()).unwrap();

        let name: String = String::from(cap.name("name").unwrap().as_str());
        if let Some(x) = cap.name("number") {
            let number = Some(x.as_str().parse().unwrap());
            return Monkey { name: name.to_string(), sum: None, number }
        } else {
            let name1: String = String::from(cap.name("name1").unwrap().as_str());
            let name2: String = String::from(cap.name("name2").unwrap().as_str());
            let operator_str: &str = cap.name("operator").unwrap().as_str();
            let operator = match operator_str {
                "+" => Operation::Add,
                "-" => Operation::Subtract,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                _ => panic!("Unrecognised {operator_str}")
            };

            let sum = Some((name1, name2, operator));

            return Monkey { name, sum, number: None };
        }
    }
}

struct Monkeys {
    monkeys: HashMap<String, Monkey>
}

impl Monkeys {
    fn contains_humn(&self, name: &String) -> bool {
        if *name == String::from("humn") {
            return true
        }

        let monkey = self.monkeys.get(name).unwrap();
        if let Some(_) = monkey.number {
            return false;
        }

        if let Some(x) = &monkey.sum {
            let (monkey_a_name, monkey_b_name, _) = x;

            return self.contains_humn(&monkey_a_name) || self.contains_humn(&monkey_b_name);
        }

        panic!("Couldn't find answer or sum on monkey");
    }

    fn find_humn_value(&self, equal: u64, name: &String) -> u64 {
        if *name == String::from("humn") {
            return equal
        }

        let monkey = self.monkeys.get(name).unwrap();
        if let Some(x) = &monkey.sum {
            let (monkey_a, monkey_b, operator) = x;
            if self.contains_humn(&monkey_a) {
                let monkey_b_val = self.get_value(&monkey_b);
                let value = match operator {
                    Operation::Add => equal - monkey_b_val,
                    Operation::Subtract => equal + monkey_b_val,
                    Operation::Multiply => equal / monkey_b_val,
                    Operation::Divide => equal * monkey_b_val,
                };
                return self.find_humn_value(value, &monkey_a);
            } else if self.contains_humn(&monkey_b) {
                let monkey_a_val = self.get_value(&monkey_a);
                let value = match operator {
                    Operation::Add => equal - monkey_a_val,
                    Operation::Subtract => monkey_a_val - equal,
                    Operation::Multiply => equal / monkey_a_val,
                    Operation::Divide => monkey_a_val / equal,
                };
                return self.find_humn_value(value, &monkey_b);
            }
        }
        panic!("humn not found");
    }   

    fn find_humn_root_equal(&self) -> u64 {
        let root = self.monkeys.get(&String::from("root")).unwrap();
        
        if let Some(x) = &root.sum {
            let (monkey_a, monkey_b, _) = x;
            if self.contains_humn(&monkey_a) {
                let value = self.get_value(&monkey_b);
                return self.find_humn_value(value, &monkey_a);
            } else if self.contains_humn(&monkey_b) {
                let value = self.get_value(&monkey_a);
                return self.find_humn_value(value, &monkey_b);
            }
        }

        panic!("humn not found");
    }

    fn get_value(&self, name: &String) -> u64 {
        let monkey = self.monkeys.get(name).unwrap();
        if let Some(x) = monkey.number {
            return x;
        }

        if let Some(x) = &monkey.sum {
            let (monkey_a_name, monkey_b_name, operator) = x;

            let monkey_a = self.get_value(&monkey_a_name);
            let monkey_b = self.get_value(&monkey_b_name);

            let answer = match operator {
                Operation::Add => monkey_a + monkey_b,
                Operation::Subtract => monkey_a - monkey_b,
                Operation::Multiply => monkey_a * monkey_b,
                Operation::Divide => monkey_a / monkey_b,
            };
            return answer;
        }

        panic!("Couldn't find answer or sum on monkey");
    }

    fn new(input: &str) -> Monkeys {
        let mut monkeys = HashMap::new();

        for line in input.lines() {
            let monkey = Monkey::new(line);
            monkeys.insert(monkey.name.clone(), monkey);
        }

        Monkeys { monkeys }
    }
}


impl Problem for DayTwentyOne {
    fn part_one(&self, input: &str) -> String {
        let monkeys = Monkeys::new(&input);
        let root = monkeys.get_value(&String::from("root"));
        format!("Root number: {root}")
    }

    fn part_two(&self, input: &str) -> String {
        let monkeys = Monkeys::new(&input);
        let humn = monkeys.find_humn_root_equal();
        format!("humn number: {humn}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_monkeys_p1() {
        let output = DayTwentyOne{}.part_one(&INPUT);
        assert_eq!(output, "Root number: 152")
    }

    #[test]
    fn test_monkeys_p2() {
        let output = DayTwentyOne{}.part_two(&INPUT);
        assert_eq!(output, "humn number: 301")
    }
}
