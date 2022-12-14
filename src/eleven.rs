use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = calc_monkey_business(&values);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = calc_monkey_business_2(&values);

    println!("{}", answer);
}

type Item = i64;

#[derive(Debug)]
struct State {
    items_by_monkey: HashMap<usize, Vec<Item>>,
}

impl State {
    fn new() -> State {
        State {
            items_by_monkey: HashMap::new(),
        }
    }

    fn from_monkeys(monkeys: &[Monkey]) -> State {
        let mut state = State::new();
        for (num, monkey) in monkeys.iter().enumerate() {
            for item in monkey.starting_items.iter() {
                state.add_item(*item, num);
            }
        }

        state
    }

    fn add_item(&mut self, item: Item, destination: usize) {
        if let Some(bucket) = self.items_by_monkey.get_mut(&destination) {
            bucket.push(item);
        } else {
            self.items_by_monkey.insert(destination, vec![item]);
        }
    }
}

#[derive(Debug)]
struct Monkey {
    starting_items: Vec<Item>,
    operation: Operation,
    rule: MonkeyRule,
}

impl Monkey {
    fn from_lines(lines: &[String]) -> Monkey {
        let starting_items: Vec<Item> = lines[1]
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        let operation = Operation::from_str(&lines[2]);
        let rule = MonkeyRule::from_lines(&lines[3..]);

        Monkey {
            starting_items,
            operation,
            rule,
        }
    }

    fn calc_new_item_value(&self, item: Item) -> Item {
        self.operation.calc(item)
    }

    fn calc_destination(&self, value: Item) -> usize {
        self.rule.apply(value)
    }
}

#[derive(Debug)]
enum Symbol {
    Old,
    Value(Item),
}

impl Symbol {
    fn from_str(s: &str) -> Symbol {
        match s {
            "old" => Symbol::Old,
            _ => Symbol::Value(s.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Symbol),
    Multiply(Symbol),
}

impl Operation {
    fn from_str(line: &str) -> Operation {
        let op = *line.split(' ').collect::<Vec<&str>>().get(6).unwrap();
        let value = Symbol::from_str(line.split(' ').last().unwrap());

        match op {
            "+" => Operation::Add(value),
            "*" => Operation::Multiply(value),
            _ => panic!(),
        }
    }

    fn calc(&self, value: Item) -> Item {
        match self {
            Operation::Add(Symbol::Old) => value + value,
            Operation::Add(Symbol::Value(v2)) => value + v2,
            Operation::Multiply(Symbol::Old) => value * value,
            Operation::Multiply(Symbol::Value(v2)) => value * v2,
        }
    }
}

#[derive(Debug)]
struct MonkeyRule {
    divisor: Item,
    true_dest: usize,
    false_dest: usize,
}

impl MonkeyRule {
    fn from_lines(lines: &[String]) -> MonkeyRule {
        let divisor = lines[0].split(' ').last().unwrap().parse().unwrap();
        let true_dest = lines[1].split(' ').last().unwrap().parse().unwrap();
        let false_dest = lines[2].split(' ').last().unwrap().parse().unwrap();

        MonkeyRule {
            divisor,
            true_dest,
            false_dest,
        }
    }

    fn apply(&self, value: Item) -> usize {
        if value % self.divisor == 0 {
            self.true_dest
        } else {
            self.false_dest
        }
    }
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut lines_buffer = vec![];
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            monkeys.push(Monkey::from_lines(&lines_buffer));
            lines_buffer.clear();
            continue;
        }

        lines_buffer.push(line);
    }

    monkeys.push(Monkey::from_lines(&lines_buffer));
    lines_buffer.clear();

    monkeys
}

fn calc_monkey_business(monkeys: &[Monkey]) -> i32 {
    let mut prev_state = State::from_monkeys(monkeys);
    let mut inspection_counts: HashMap<usize, i32> = HashMap::new();
    for _ in 0..20 {
        let mut new_state = State::new();
        for (num, monkey) in monkeys.iter().enumerate() {
            for item in prev_state
                .items_by_monkey
                .get(&num)
                .cloned()
                .unwrap_or_default()
                .iter()
            {
                let new_value = monkey.calc_new_item_value(*item) / 3;
                let destination = monkey.calc_destination(new_value);

                if destination <= num {
                    new_state.add_item(new_value, destination);
                } else {
                    prev_state.add_item(new_value, destination);
                }

                *inspection_counts.entry(num).or_insert(0) += 1;
            }
        }

        prev_state = new_state;
    }

    let mut counts = inspection_counts.values().cloned().collect::<Vec<i32>>();
    counts.sort();
    counts.reverse();

    counts.iter().take(2).product()
}

fn calc_monkey_business_2(monkeys: &[Monkey]) -> i64 {
    let resolution: Item = monkeys.iter().map(|m| m.rule.divisor).product();

    let mut prev_state = State::from_monkeys(monkeys);
    let mut inspection_counts: HashMap<usize, i64> = HashMap::new();
    for _ in 0..10000 {
        let mut new_state = State::new();
        for (num, monkey) in monkeys.iter().enumerate() {
            for item in prev_state
                .items_by_monkey
                .get(&num)
                .cloned()
                .unwrap_or_default()
                .iter()
            {
                let mut new_value = monkey.calc_new_item_value(*item);
                let destination = monkey.calc_destination(new_value);

                if new_value > resolution {
                    new_value %= resolution;
                }

                if destination <= num {
                    new_state.add_item(new_value, destination);
                } else {
                    prev_state.add_item(new_value, destination);
                }

                *inspection_counts.entry(num).or_insert(0) += 1;
            }
        }

        prev_state = new_state;
    }

    let mut counts: Vec<i64> = inspection_counts.values().cloned().collect::<Vec<i64>>();
    counts.sort();
    counts.reverse();

    counts.iter().take(2).product()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::fs::File;

    #[test]
    fn test_part_1_example() {
        let f = File::open("inputs/eleven_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 10605;
        let actual = calc_monkey_business(&values);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_1_actual() {
        let f = File::open("inputs/eleven.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 119715;
        let actual = calc_monkey_business(&values);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_2_example() {
        let f = File::open("inputs/eleven_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 2713310158;
        let actual = calc_monkey_business_2(&values);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_2_actual() {
        let f = File::open("inputs/eleven.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 18085004878;
        let actual = calc_monkey_business_2(&values);

        assert_eq!(expected, actual)
    }
}
