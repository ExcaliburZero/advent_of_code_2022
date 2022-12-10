use std::io::prelude::*;
use std::io::{self, BufReader};

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

pub fn part_one() {
    let numbers = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_value_assume_true(&numbers);

    println!("{}", answer);
}

pub fn part_two() {
    let numbers = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_value_assume_result(&numbers);

    println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> Vec<(char, char)> {
    let mut entries: Vec<(char, char)> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(' ');

        entries.push((
            parts.next().unwrap().chars().into_iter().next().unwrap(),
            parts.next().unwrap().chars().into_iter().next().unwrap(),
        ));
    }

    entries
}

fn find_value_assume_true(guide: &[(char, char)]) -> i32 {
    let mut total = 0;

    for (a, b) in guide.iter() {
        total += match find_result(*a, *b) {
            Res::Draw => 3,
            Res::Win => 6,
            Res::Loss => 0,
        };

        total += match *b {
            'X' => ROCK,
            'Y' => PAPER,
            'Z' => SCISSORS,
            _ => panic!(),
        };
    }

    total
}

fn find_result(a: char, b: char) -> Res {
    match (a, b) {
        ('A', 'X') => Res::Draw,
        ('B', 'Y') => Res::Draw,
        ('C', 'Z') => Res::Draw,
        ('A', 'Y') => Res::Win,
        ('A', 'Z') => Res::Loss,
        ('B', 'X') => Res::Loss,
        ('B', 'Z') => Res::Win,
        ('C', 'X') => Res::Win,
        ('C', 'Y') => Res::Loss,
        _ => panic!(),
    }
}

fn find_resulting_value(a: char, b: char) -> i32 {
    match (a, b) {
        ('A', 'X') => SCISSORS,
        ('A', 'Y') => ROCK,
        ('A', 'Z') => PAPER,
        ('B', 'X') => ROCK,
        ('B', 'Y') => PAPER,
        ('B', 'Z') => SCISSORS,
        ('C', 'X') => PAPER,
        ('C', 'Y') => SCISSORS,
        ('C', 'Z') => ROCK,
        _ => panic!(),
    }
}

enum Res {
    Win,
    Loss,
    Draw,
}

fn find_value_assume_result(guide: &[(char, char)]) -> i32 {
    let mut total = 0;

    for (a, b) in guide.iter() {
        total += find_resulting_value(*a, *b);

        total += match *b {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!(),
        };
    }

    total
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::fs::File;

    #[test]
    fn test_part_1_example() {
        let f = File::open("inputs/two_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 15;
        let actual = find_value_assume_true(&values);

        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn test_part_1_actual() {
        let f = File::open("inputs/two.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 11475;
        let actual = find_value_assume_true(&values);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2_example() {
        let f = File::open("inputs/two_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 12;
        let actual = find_value_assume_result(&values);

        assert_eq!(expected, actual);
    }

    #[ignore]
    #[test]
    fn test_part_2_actual() {
        let f = File::open("inputs/two.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 16862;
        let actual = find_value_assume_result(&values);

        assert_eq!(expected, actual);
    }
}
