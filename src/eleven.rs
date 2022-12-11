use num_bigint::{BigUint, ToBigUint};
use num_traits::Zero;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let mut values = read_input(&mut BufReader::new(io::stdin()));
    let answer = calc_monkey_business(&mut values);

    println!("{}", answer);
}

pub fn part_two() {
    let mut values = read_input(&mut BufReader::new(io::stdin()));
    let answer = calc_monkey_business_2(&mut values);

    println!("{}", answer);
}

type Item = BigUint;

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
                state.add_item(item.clone(), num);
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
        let op = line
            .split(' ')
            .collect::<Vec<&str>>()
            .get(6)
            .unwrap()
            .clone();
        let value = Symbol::from_str(line.split(' ').last().unwrap());

        match op {
            "+" => Operation::Add(value),
            "*" => Operation::Multiply(value),
            _ => panic!(),
        }
    }

    fn calc(&self, value: Item) -> Item {
        match self {
            Operation::Add(Symbol::Old) => value.clone() + value.clone(),
            Operation::Add(Symbol::Value(v2)) => value + v2,
            Operation::Multiply(Symbol::Old) => value.clone() * value.clone(),
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
        if value % self.divisor.clone() == Zero::zero() {
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

fn calc_monkey_business(monkeys: &mut [Monkey]) -> i32 {
    let mut prev_state = State::from_monkeys(monkeys);
    let mut inspection_counts: HashMap<usize, i32> = HashMap::new();
    for _ in 0..20 {
        println!("{prev_state:?}");

        let mut new_state = State::new();
        for (num, monkey) in monkeys.iter().enumerate() {
            println!("Monkey {num}:");
            for item in prev_state
                .items_by_monkey
                .get(&num)
                .cloned()
                .or(Some(vec![]))
                .unwrap()
                .iter()
            {
                println!("  Inpsects {item}");

                let new_value = monkey.calc_new_item_value(item.clone()) / 3.to_biguint().unwrap();
                let destination = monkey.calc_destination(new_value.clone());

                //println!("  now {new_value.clone()}");
                println!("  to {destination}");

                if destination <= num {
                    new_state.add_item(new_value.clone(), destination);
                } else {
                    prev_state.add_item(new_value.clone(), destination);
                }

                if !inspection_counts.contains_key(&num) {
                    inspection_counts.insert(num, 0);
                }
                *inspection_counts.get_mut(&num).unwrap() += 1;
            }

            //println!("\t{prev_state:?}");
        }

        prev_state = new_state;
    }

    println!("{prev_state:?}");

    let mut counts = inspection_counts.values().cloned().collect::<Vec<i32>>();
    counts.sort();
    counts.reverse();
    println!("{inspection_counts:?}");
    println!("{counts:?}");
    counts.iter().take(2).product()
}

fn calc_monkey_business_2(monkeys: &mut [Monkey]) -> i64 {
    let mut prev_state = State::from_monkeys(monkeys);
    let mut inspection_counts: HashMap<usize, i64> = HashMap::new();
    println!(" initial: {prev_state:?}");
    let resolution: BigUint = monkeys.iter().map(|m| m.rule.divisor.clone()).product();
    for r in 0..10000 {
        //for r in 0..50 {
        //println!("{r}");

        let mut new_state = State::new();
        for (num, monkey) in monkeys.iter().enumerate() {
            //println!("Monkey {num}:");
            for item in prev_state
                .items_by_monkey
                .get(&num)
                .cloned()
                .or(Some(vec![]))
                .unwrap()
                .iter()
            {
                //println!("  Inpsects {item}");

                let mut new_value = monkey.calc_new_item_value(item.clone());
                //let new_value = monkey.calc_new_item_value(item.clone()) / 3.to_biguint().unwrap();

                let destination = monkey.calc_destination(new_value.clone());

                //println!("  now {new_value}");
                //println!("  to {destination}");

                if new_value > resolution.clone() {
                    new_value = new_value % resolution.clone();
                }

                if destination <= num {
                    new_state.add_item(new_value, destination);
                } else {
                    prev_state.add_item(new_value, destination);
                }

                if !inspection_counts.contains_key(&num) {
                    inspection_counts.insert(num, 0);
                }
                *inspection_counts.get_mut(&num).unwrap() += 1;
            }

            //println!("\t{prev_state:?}");
        }

        prev_state = new_state;

        if vec![0, 19].contains(&r) {
            println!("\t{prev_state:?}");
            println!("{r}: {inspection_counts:?}");
        }
    }

    //println!("{prev_state:?}");

    let mut counts: Vec<i64> = inspection_counts.values().cloned().collect::<Vec<i64>>();
    counts.sort();
    counts.reverse();
    println!("{inspection_counts:?}");
    println!("{counts:?}");
    counts.iter().take(2).product()
}

/*
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::fs::File;

    #[test]
    fn test_part_1_example() {
        let f = File::open("inputs/six_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 7;
        let actual = find_start_marker(&values[0]);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_1_actual() {
        let f = File::open("inputs/six.txt").unwrap();
        let values = read_
fn find_start_marker(values: &str) -> i32 {
    let mut buffer = Vec::new();
    for (i, c) in values.chars().enumerate() {
        if buffer.len() == 4 {
            buffer.reverse();
            buffer.pop();
            buffer.reverse();
        }

        buffer.push(c);

        let unique_chars: HashSet<char> = buffer.iter().cloned().collect();
        if unique_chars.len() == 4 {
            return i as i32 + 1;
        }
    }

    panic!()
}

fn find_start_marker_2(values: &str) -> i32 {
    let mut buffer = Vec::new();
    for (i, c) in values.chars().enumerate() {
        if buffer.len() == 14 {
            buffer.reverse();
            buffer.pop();
            buffer.reverse();
        }

        buffer.push(c);

        let unique_chars: HashSet<char> = buffer.iter().cloned().collect();
        if unique_chars.len() == 14 {
            return i as i32 + 1;
        }
    }

    panic!()
}
tart_marker(&values[0]);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_2_example() {
        let f = File::open("inputs/six_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 19;
        let actual = find_start_marker_2(&values[0]);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_2_actual() {
        let f = File::open("inputs/six.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 3716;
        let actual = find_start_marker_2(&values[0]);

        assert_eq!(expected, actual)
    }
}
*/
