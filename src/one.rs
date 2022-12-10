use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let numbers = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_max_calories(&numbers);

    println!("{}", answer);
}

pub fn part_two() {
    let numbers = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_top_three_calories(&numbers);

    println!("{}", answer);
}

fn find_top_three_calories(elves: &[Vec<i32>]) -> i32 {
    let mut totals: Vec<i32> = elves.iter().map(|e| e.iter().sum()).collect();

    totals.sort();

    totals.iter().rev().take(3).sum()
}

fn find_max_calories(elves: &[Vec<i32>]) -> i32 {
    let mut largest = 0;

    for elve in elves.iter() {
        let total = elve.iter().sum();

        if total > largest {
            largest = total;
        }
    }

    largest
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> Vec<Vec<i32>> {
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    numbers.push(Vec::new());
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            numbers.push(Vec::new());

            continue;
        }

        let number: i32 = line.parse().unwrap();

        numbers.iter_mut().last().unwrap().push(number);
    }

    numbers
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::fs::File;

    #[test]
    fn test_part_1_example() {
        let f = File::open("inputs/one_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 24000;
        let actual = find_max_calories(&values);

        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn test_part_1_actual() {
        let f = File::open("inputs/one.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 74394;
        let actual = find_max_calories(&values);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2_example() {
        let f = File::open("inputs/one_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 45000;
        let actual = find_top_three_calories(&values);

        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn test_part_2_actual() {
        let f = File::open("inputs/one.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 212836;
        let actual = find_top_three_calories(&values);

        assert_eq!(expected, actual);
    }
}
