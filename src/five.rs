use regex::Regex;
use std::io::prelude::*;
use std::io::{self, BufReader};

type Crate = Vec<char>;

pub fn part_one() {
    let (mut crates, moves) = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_top_crates_after_moves(&mut crates, &moves);

    println!("{}", answer);
}

pub fn part_two() {
    let (mut crates, moves) = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_top_crates_after_moves_2(&mut crates, &moves);

    println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> (Vec<Crate>, Vec<Move>) {
    let mut crates: Vec<Crate> = Vec::new();
    let mut moves: Vec<Move> = Vec::new();
    let mut at_moves = false;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            // Hanlde the line separating the crates from the moves
            at_moves = true;
            continue;
        }

        if line.chars().collect::<Vec<char>>()[1] == '1' {
            // Skip the crate indices line, since we can recalulate them as needed and thus don't need to store them
            continue;
        }

        if !at_moves {
            if crates.is_empty() {
                // Haven't created the crates yet, so let's calculate how many we need and create them as empty
                let num_crates = (line.len() as f32 / 4.0).ceil() as usize;
                for _ in 0..num_crates {
                    crates.push(Vec::new());
                }
            }

            // Populate all of the crates that have entries in this row of text
            for (i, cr) in crates.iter_mut().enumerate() {
                let c = line.chars().collect::<Vec<char>>()[i * 4 + 1];
                if c == ' ' {
                    continue;
                }

                cr.push(c);
            }
        } else {
            moves.push(Move::from_line(&line));
        }
    }

    // Reverse all the crates so higher (top) items are at the end of the Vec
    for cr in crates.iter_mut() {
        cr.reverse();
    }

    (crates, moves)
}

#[derive(Debug)]
struct Move {
    count: i32,
    src: usize,
    dest: usize,
}

impl Move {
    fn from_line(line: &str) -> Move {
        let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
        let matches = re.captures_iter(line).next().unwrap();

        Move {
            count: matches.get(1).unwrap().as_str().parse().unwrap(),
            src: matches.get(2).unwrap().as_str().parse().unwrap(),
            dest: matches.get(3).unwrap().as_str().parse().unwrap(),
        }
    }
}

fn find_top_crates_after_moves(state: &mut [Crate], moves: &[Move]) -> String {
    for m in moves.iter() {
        preform_move(state, m);
    }

    state.iter().map(|cr| cr.last().unwrap()).collect()
}

fn preform_move(state: &mut [Crate], m: &Move) {
    for _ in 0..m.count {
        let value = state[m.src - 1].pop().unwrap();
        state[m.dest - 1].push(value);
    }
}

fn find_top_crates_after_moves_2(state: &mut [Crate], moves: &[Move]) -> String {
    for m in moves.iter() {
        preform_move_2(state, m);
    }

    state.iter().map(|cr| cr.last().unwrap()).collect()
}

fn preform_move_2(state: &mut [Crate], m: &Move) {
    let mut stack = Vec::new();
    for _ in 0..m.count {
        let value = state[m.src - 1].pop().unwrap();
        stack.push(value);
    }

    stack.reverse();

    for v in stack.iter() {
        state[m.dest - 1].push(*v);
    }
}

/*fn print_state(state: &[Crate]) {
    for cr in state.iter() {
        for c in cr.iter() {
            print!("{} ", c);
        }
        println!();
    }
    println!();
}*/

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::fs::File;

    #[test]
    fn test_part_1_example() {
        let f = File::open("inputs/five_example.txt").unwrap();
        let (mut crates, moves) = read_input(&mut BufReader::new(f));

        let expected = "CMZ".to_string();
        let actual = find_top_crates_after_moves(&mut crates, &moves);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_1_actual() {
        let f = File::open("inputs/five.txt").unwrap();
        let (mut crates, moves) = read_input(&mut BufReader::new(f));

        let expected = "JRVNHHCSJ".to_string();
        let actual = find_top_crates_after_moves(&mut crates, &moves);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_2_example() {
        let f = File::open("inputs/five_example.txt").unwrap();
        let (mut crates, moves) = read_input(&mut BufReader::new(f));

        let expected = "MCD".to_string();
        let actual = find_top_crates_after_moves_2(&mut crates, &moves);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_2_actual() {
        let f = File::open("inputs/five.txt").unwrap();
        let (mut crates, moves) = read_input(&mut BufReader::new(f));

        let expected = "GNFBSBJLH".to_string();
        let actual = find_top_crates_after_moves_2(&mut crates, &moves);

        assert_eq!(expected, actual)
    }
}
