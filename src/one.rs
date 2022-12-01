use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let numbers = read_input();
    let answer = find_max_calories(&numbers);

    println!("{}", answer);
}

pub fn part_two() {
    let numbers = read_input();
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

fn read_input() -> Vec<Vec<i32>> {
    let stdin = io::stdin();

    let mut numbers: Vec<Vec<i32>> = Vec::new();
    numbers.push(Vec::new());
    for line in stdin.lock().lines() {
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
