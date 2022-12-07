use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_contained_ranges(&values);

    println!("{}", answer);
}

pub fn part_two() {
    let numbers = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_overlapping_ranges(&numbers);

    println!("{}", answer);
}

type Range = (i32, i32);

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> Vec<(Range, Range)> {
    let mut numbers: Vec<((i32, i32), (i32, i32))> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();

        let mut parts = line.split(',');

        let mut part_1 = parts.next().unwrap().split('-');
        let mut part_2 = parts.next().unwrap().split('-');

        let a: (i32, i32) = (
            part_1.next().unwrap().parse().unwrap(),
            part_1.next().unwrap().parse().unwrap(),
        );
        let b: (i32, i32) = (
            part_2.next().unwrap().parse().unwrap(),
            part_2.next().unwrap().parse().unwrap(),
        );

        numbers.push((a, b));
    }

    numbers
}

fn find_contained_ranges(ranges: &[(Range, Range)]) -> i32 {
    let mut total = 0;

    for (a, b) in ranges.iter() {
        if fully_contains(a, b) || fully_contains(b, a) {
            total += 1;
        }
    }

    total
}

fn find_overlapping_ranges(ranges: &[(Range, Range)]) -> i32 {
    let mut total = 0;

    for (a, b) in ranges.iter() {
        if overlaps(a, b) {
            total += 1;
        }
    }

    total
}

fn fully_contains(a: &Range, b: &Range) -> bool {
    let (a_s, a_e) = a;
    let (b_s, b_e) = b;

    a_s <= b_s && a_e >= b_e
}

fn overlaps(a: &Range, b: &Range) -> bool {
    let (a_s, a_e) = a;
    let (b_s, b_e) = b;

    a_s <= b_e && b_s <= a_e
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::fs::File;

    #[test]
    fn test_part_1_example() {
        let f = File::open("inputs/four_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 2;
        let actual = find_contained_ranges(&values);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_1_actual() {
        let f = File::open("inputs/four.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 485;
        let actual = find_contained_ranges(&values);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_2_example() {
        let f = File::open("inputs/four_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 4;
        let actual = find_overlapping_ranges(&values);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_2_actual() {
        let f = File::open("inputs/four.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 857;
        let actual = find_overlapping_ranges(&values);

        assert_eq!(expected, actual)
    }
}
