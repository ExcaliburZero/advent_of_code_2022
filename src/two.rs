use std::io;
use std::io::prelude::*;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

pub fn part_one() {
    let numbers = read_input();
    let answer = find_value_assume_true(&numbers);

    println!("{}", answer);
}

pub fn part_two() {
    let numbers = read_input();
    let answer = find_value_assume_result(&numbers);

    println!("{}", answer);
}

fn read_input() -> Vec<(char, char)> {
    let stdin = io::stdin();

    let mut entries: Vec<(char, char)> = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut parts = line.split(" ");

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
