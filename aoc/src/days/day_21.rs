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
    fn get_value(&self, name: &String) -> u64 {
        let monkey = self.monkeys.get(name).unwrap();
        if let Some(x) = monkey.number {
            return x;
        }

        if let Some(x) = &monkey.sum {
            let (monkey_a_name, monkey_b_name, operator) = x;

            let monkey_a = self.get_value(monkey_a_name);
            let monkey_b = self.get_value(monkey_b_name);

            let answer = match operator {
                Operation::Add => monkey_a + monkey_b,
                Operation::Subtract => monkey_a - monkey_b,
                Operation::Multiply => monkey_a * monkey_b,
                Operation::Divide => monkey_a / monkey_b,
            };
            //@@@ mutable issues - need to re-get monkey.number = Some(answer);
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

    fn part_two(&self, _input: &str) -> String {
        format!("Not yet implemented")
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
    fn test_cipher_p1() {
        let output = DayTwentyOne{}.part_one(&INPUT);
        assert_eq!(output, "Root number: 152")
    }
}
