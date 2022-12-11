use crate::problem::Problem;
use std::collections::HashMap;
use regex::Regex;

pub struct DayEleven {}

enum Operation {
    Multiply,
    Add
}

struct Monkey {
    num: u8, 
    items_inspected: u64,
    items: Vec<u64>,
    operation: Operation,
    change: Option<u8>,
    divisible: u8,
    true_monkey: u8, 
    false_monkey: u8,
    worry_level_divide: bool
}

impl Monkey {
    pub fn play_round(&mut self) -> Vec<(u64, u8)> {
        let mut throws = Vec::new();
        for item in &self.items {
            self.items_inspected += 1;
            let change: u64 = match self.change {
                Some(x) => x as u64,
                None => *item,
            };
            let mut worry_level = match self.operation {
                Operation::Multiply => item * change,
                Operation::Add => item + change,
            };
            if self.worry_level_divide {
                worry_level = worry_level/3;
            }

            if worry_level % self.divisible as u64 == 0 {
                throws.push((worry_level, self.true_monkey));
            } else {
                throws.push((worry_level, self.false_monkey));
            }
        }

        // Empty items 
        self.items = Vec::new();

        throws
    }

    pub fn throw(&mut self, item: u64) {
        self.items.push(item)
    }

    pub fn new(input: &str, worry_level_divide: bool) -> Monkey {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Monkey\s(?P<monkey_num>[0-9]+):\n\s*Starting\sitems:(?P<items>[0-9,\s]+)\n\s*Operation:\snew\s=\sold\s(?P<operation>[+*])\s(?P<change>(old|[0-9]+))\n\s*Test:\sdivisible\sby\s(?P<divisible>[0-9]+)\n\s*If\strue:\sthrow\sto\smonkey\s(?P<true_monkey>[0-9]+)\n\s*If\sfalse:\sthrow\sto\smonkey\s(?P<false_monkey>[0-9]+)$").unwrap();
        };
        let cap = RE.captures(input).unwrap();
        let monkey_num: u8 = cap.name("monkey_num").unwrap().as_str().parse().unwrap();
        let items: Vec<u64> = cap.name("items").unwrap().as_str().split(",").map(|x| x.trim().parse().unwrap()).collect();
        let operation_str: &str = cap.name("operation").unwrap().as_str();
        let operation = match operation_str {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("Didn't recognise operation {operation_str}"),
        };
        let change_str: &str = cap.name("change").unwrap().as_str();
        let change = match change_str {
            "old" => None,
            _ => Some(change_str.parse().unwrap())
        };
        let divisible: u8 = cap.name("divisible").unwrap().as_str().parse().unwrap();
        let true_monkey: u8 = cap.name("true_monkey").unwrap().as_str().parse().unwrap();
        let false_monkey: u8 = cap.name("false_monkey").unwrap().as_str().parse().unwrap();

        Monkey { 
            num:monkey_num,
            items_inspected:0, items:items, 
            operation:operation, change:change,
            divisible:divisible, true_monkey:true_monkey, 
            false_monkey:false_monkey, worry_level_divide:worry_level_divide
        }
    }
}

struct MonkeyThrower {
    monkeys: HashMap<u8, Monkey>,
    lcm: u64
}

impl MonkeyThrower {
    pub fn score(&self) -> u64 {
        let mut monkey_scores = Vec::new();
        for monkey in self.monkeys.values() {
            monkey_scores.push(monkey.items_inspected);
        }
        monkey_scores.sort();
        return monkey_scores[self.monkeys.len() - 1] * monkey_scores[self.monkeys.len() - 2]
    }

    pub fn run(&mut self, round_num: u32) {
        let mut monkey_keys: Vec<u8> = self.monkeys.keys().map(|x| *x).collect();
        monkey_keys.sort();
        for _ in 0..round_num {
            for monkey_key in &monkey_keys {
                let monkey = self.monkeys.get_mut(&monkey_key).unwrap();

                let throws = monkey.play_round();
                for (item, receiving_monkey_key) in throws {
                    let receiving_monkey = self.monkeys.get_mut(&receiving_monkey_key).unwrap();
                    // Manage overflow
                    let reduced_item = item % self.lcm;
                    receiving_monkey.throw(reduced_item);
                }
            }
        }
    }

    pub fn new(input: &str, worry_level_divide: bool) -> MonkeyThrower {
        let mut lcm: u64 = 1;
        let mut monkeys = HashMap::new();
        for monkey_input in input.split("\n\n") {
            let monkey = Monkey::new(monkey_input, worry_level_divide);
            lcm = lcm * monkey.divisible as u64;
            monkeys.insert(monkey.num, monkey);
        }
        
        MonkeyThrower { monkeys:monkeys, lcm:lcm }
    }
}


impl Problem for DayEleven {
    fn part_one(&self, input: &str) -> String {
        let mut monkey_thrower = MonkeyThrower::new(&input, true);
        monkey_thrower.run(20);
        let score = monkey_thrower.score();
        format!("Monkey business: {score}")
    }

    fn part_two(&self, input: &str) -> String {
        let mut monkey_thrower = MonkeyThrower::new(&input, false);
        monkey_thrower.run(10000);
        let score = monkey_thrower.score();
        format!("Monkey business: {score}")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
If true: throw to monkey 0
If false: throw to monkey 1";

    #[test]
    fn test_monkey_thrower_p1() {
        let output = DayEleven{}.part_one(&INPUT);
        assert_eq!(output, "Monkey business: 10605")
    }

    #[test]
    fn test_monkey_thrower_p2() {
        let output = DayEleven{}.part_two(&INPUT);
        assert_eq!(output, "Monkey business: 2713310158")
    }
}
