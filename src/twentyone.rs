use std::collections::HashMap;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_root_value(&values);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    //let answer = find_start_marker_2(&values[0]);

    //println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> HashMap<MonkeyName, Formula> {
    let mut monkeys = HashMap::new();
    for line in reader.lines() {
        let monkey = parse_monkey(&line.unwrap());
        monkeys.insert(monkey.0, monkey.1);
    }

    monkeys
}

type Monkey = (String, Formula);
type MonkeyName = String;

fn parse_monkey(line: &str) -> Monkey {
    let mut parts = line.split(": ");
    let name = parts.next().unwrap();
    let formula = Formula::from_str(parts.next().unwrap());

    (name.to_string(), formula)
}

fn find_root_value(monkeys: &HashMap<MonkeyName, Formula>) -> i64 {
    let mut cache = MonkeyCache::from_monkeys(monkeys);

    cache.get(&"root".to_string())
}

struct MonkeyCache {
    monkeys: HashMap<MonkeyName, Formula>,
    cache: HashMap<MonkeyName, i64>
}

impl MonkeyCache {
    fn from_monkeys(monkeys: &HashMap<MonkeyName, Formula>) -> MonkeyCache {
        MonkeyCache {
            monkeys: monkeys.clone(),
            cache: HashMap::new(),
        }
    }

    fn get(&mut self, monkey: &MonkeyName) -> i64 {
        if let Some(value) = self.cache.get(monkey) {
            *value
        } else {
            let value = self.calc_value(monkey);
            self.cache.insert(monkey.to_string(), value);

            value
        }
    }

    fn calc_value(&mut self, monkey: &MonkeyName) -> i64 {
        match self.monkeys.get(monkey).unwrap().clone() {
            Formula::Constant(value) => value,
            Formula::Calculation(monkey_1, operation, monkey_2) => {
                let left = self.get(&monkey_1);
                let right = self.get(&monkey_2);
                operation.apply(left, right)
            },
        }
    }
}

#[derive(Debug, Clone)]
enum Formula {
    Constant(i64),
    Calculation(MonkeyName, Operation, MonkeyName)
}

impl Formula {
    fn from_str(s: &str) -> Formula {
        if let Ok(number) = s.parse::<i64>() {
            return Formula::Constant(number);
        }

        let mut parts = s.split(' ');
        let monkey_1 = parts.next().unwrap();
        let operation = Operation::from_char(parts.next().unwrap().chars().next().unwrap());
        let monkey_2 = parts.next().unwrap();

        Formula::Calculation(monkey_1.to_string(), operation, monkey_2.to_string())
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Addition, Subtraction, Multiplication, Division
}

impl Operation {
    fn from_char(c: char) -> Operation {
        match c {
            '+' => Operation::Addition,
            '-' => Operation::Subtraction,
            '*' => Operation::Multiplication,
            '/' => Operation::Division,
            _ => panic!(),
        }
    }

    fn apply(&self, left: i64, right: i64) -> i64 {
        use Operation::*;

        match self {
            Addition => left + right,
            Subtraction => left - right,
            Multiplication => left * right,
            Division => left / right,
        }
    }
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
        let values = read_input(&mut BufReader::new(f));

        let expected = 1287;
        let actual = find_start_marker(&values[0]);

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