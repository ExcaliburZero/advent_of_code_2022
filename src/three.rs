use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

pub fn part_one() {
    let values = read_input();
    let answer = sum_priorities(&values);

    println!("{}", answer);
}

pub fn part_two() {
    let numbers = read_input();
    let answer = sum_priorities_of_badges(&numbers);

    println!("{}", answer);
}

fn read_input() -> Vec<String> {
    let stdin = io::stdin();

    let mut numbers: Vec<String> = Vec::new();
    for line in stdin.lock().lines() {
        numbers.push(line.unwrap().to_string());
    }

    numbers
}

fn sum_priorities(values: &[String]) -> i32 {
    let mut total = 0;

    for value in values.iter() {
        let parts = split_rucksack(value);

        let set_a: HashSet<char> = parts.0.chars().collect();
        let set_b: HashSet<char> = parts.1.chars().collect();

        let intersection = set_a.intersection(&set_b).next().unwrap();

        total += get_priority(*intersection);
    }

    total
}

fn split_rucksack(value: &String) -> (String, String) {
    let split_index = value.len() / 2;

    (
        value[0..split_index].to_string(),
        value[split_index..].to_string(),
    )
}

fn get_priority(c: char) -> i32 {
    if c.is_lowercase() {
        ((c as u32 - 'a' as u32) + 1) as i32
    } else {
        ((c as u32 - 'A' as u32) + 1 + 26) as i32
    }
}

fn sum_priorities_of_badges(values: &[String]) -> i32 {
    let mut total = 0;

    for i in 0..(values.len() / 3) {
        let a = i * 3;
        let b = a + 1;
        let c = a + 2;

        let set_a: HashSet<char> = values[a].chars().collect();
        let set_b: HashSet<char> = values[b].chars().collect();
        let set_c: HashSet<char> = values[c].chars().collect();

        let intersection_ab: HashSet<char> = set_a.intersection(&set_b).cloned().collect();
        let intersection_abc: HashSet<char> =
            intersection_ab.intersection(&set_c).cloned().collect();

        let intersection = intersection_abc.iter().next().unwrap();

        total += get_priority(*intersection);
    }

    total
}
