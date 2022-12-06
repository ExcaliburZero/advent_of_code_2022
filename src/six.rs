use std::collections::HashSet;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_start_marker(&values[0]);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_start_marker_2(&values[0]);

    println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> Vec<String> {
    let mut numbers: Vec<String> = Vec::new();
    for line in reader.lines() {
        numbers.push(line.unwrap().to_string());
    }

    numbers
}

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

    #[test]
    fn test_part_2_actual() {
        let f = File::open("inputs/six.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 3716;
        let actual = find_start_marker_2(&values[0]);

        assert_eq!(expected, actual)
    }
}
