use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_root_value(&values);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = calc_human_value(&values, &"root".to_string(), &"humn".to_string());

    println!("{}", answer);
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

fn calc_human_value(
    monkeys: &HashMap<MonkeyName, Formula>,
    root_name: &MonkeyName,
    human_name: &MonkeyName,
) -> i64 {
    let (left, right) = monkeys.get(root_name).unwrap().get_args().unwrap();

    let left = get_formula2(&left, monkeys, human_name).simplified();
    let right = get_formula2(&right, monkeys, human_name).simplified();

    let (variable, other) = match (left, right) {
        (Formula2::Constant(other), variable) => (variable, other),
        (variable, Formula2::Constant(other)) => (variable, other),
        (_, _) => panic!(),
    };

    variable.solve(other)
}

fn get_formula2(
    name: &MonkeyName,
    monkeys: &HashMap<MonkeyName, Formula>,
    human_name: &MonkeyName,
) -> Formula2 {
    if name == human_name {
        return Formula2::Variable();
    }

    let formula_1 = monkeys.get(name).unwrap();
    match formula_1 {
        Formula::Constant(value) => Formula2::Constant(*value),
        Formula::Calculation(left, operation, right) => Formula2::Calculation(
            Box::new(get_formula2(left, monkeys, human_name)),
            *operation,
            Box::new(get_formula2(right, monkeys, human_name)),
        ),
    }
}

struct MonkeyCache {
    monkeys: HashMap<MonkeyName, Formula>,
    cache: HashMap<MonkeyName, i64>,
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
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Formula {
    Constant(i64),
    Calculation(MonkeyName, Operation, MonkeyName),
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

    fn get_args(&self) -> Option<(MonkeyName, MonkeyName)> {
        match self {
            Formula::Constant(_) => None,
            Formula::Calculation(left, _, right) => Some((left.clone(), right.clone())),
        }
    }
}

#[derive(Debug, Clone)]
enum Formula2 {
    Constant(i64),
    Calculation(Box<Formula2>, Operation, Box<Formula2>),
    Variable(),
}

impl Formula2 {
    fn simplified(&self) -> Formula2 {
        use Formula2::*;

        match self {
            Calculation(left, operation, right) => {
                let left = left.simplified();
                let right = right.simplified();

                match (left, right) {
                    (Constant(left), Constant(right)) => {
                        Formula2::Constant(operation.apply(left, right))
                    }
                    (left, right) => {
                        Formula2::Calculation(Box::new(left), *operation, Box::new(right))
                    }
                }
            }
            const_or_var => const_or_var.clone(),
        }
    }

    fn solve(&self, other: i64) -> i64 {
        use Formula2::*;

        match self {
            Variable() => other,
            Constant(_) => panic!(),
            Calculation(left, operation, right) => {
                let (variable, new_other) = match (*left.clone(), *right.clone()) {
                    (Constant(constant), variable) => {
                        let other = match *operation {
                            Operation::Addition => other - constant,
                            Operation::Subtraction => -(other - constant),
                            Operation::Multiplication => other / constant,
                            Operation::Division => constant / other,
                        };

                        (variable, other)
                    }
                    (variable, Constant(constant)) => {
                        let other = match *operation {
                            Operation::Addition => other - constant,
                            Operation::Subtraction => other + constant,
                            Operation::Multiplication => other / constant,
                            Operation::Division => other * constant,
                        };

                        (variable, other)
                    }
                    (_, _) => panic!(),
                };

                variable.solve(new_other)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
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
