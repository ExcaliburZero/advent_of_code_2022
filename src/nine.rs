use std::collections::HashSet;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = count_tail_visits(&values, 0);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = count_tail_visits(&values, 8);

    println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for line in reader.lines() {
        moves.push(Move::from_str(&line.unwrap()));
    }

    moves
}

fn count_tail_visits(moves: &[Move], num_segments: usize) -> i32 {
    let mut state = State::new(num_segments);

    for m in moves.iter() {
        state.apply_move(m)
    }

    state.known_tail_locations.len() as i32
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Position {
    row: i32,
    column: i32,
}

impl Position {
    fn new(r: i32, c: i32) -> Position {
        Position { row: r, column: c }
    }

    fn diff(&self, other: &Position) -> DistanceVector {
        DistanceVector {
            row_diff: other.row - self.row,
            column_diff: other.column - self.column,
        }
    }

    fn moved(&self, vector: &DistanceVector) -> Position {
        Position::new(self.row + vector.row_diff, self.column + vector.column_diff)
    }
}

#[derive(Debug)]
struct DistanceVector {
    row_diff: i32,
    column_diff: i32,
}

impl DistanceVector {
    fn new(r: i32, c: i32) -> DistanceVector {
        DistanceVector {
            row_diff: r,
            column_diff: c,
        }
    }

    fn euclidean_distance(&self) -> f32 {
        f32::sqrt((self.row_diff.pow(2) + self.column_diff.pow(2)) as f32)
    }

    fn to_diagonal_move(&self) -> DistanceVector {
        let row_diff = self.row_diff / self.row_diff.abs();
        let column_diff = self.column_diff / self.column_diff.abs();

        DistanceVector::new(row_diff, column_diff)
    }
}

#[derive(Debug)]
struct State {
    head: Position,
    segments: Vec<Position>,
    tail: Position,
    known_tail_locations: HashSet<Position>,
}

impl State {
    fn new(num_segments: usize) -> State {
        let mut known_tail_locations = HashSet::new();

        known_tail_locations.insert(Position::new(0, 0));

        State {
            head: Position::new(0, 0),
            segments: vec![Position::new(0, 0); num_segments],
            tail: Position::new(0, 0),
            known_tail_locations,
        }
    }

    fn apply_move(&mut self, m: &Move) {
        for _ in 0..m.distance {
            self.head = self.head.moved(&m.to_distance_vector());

            let num_segments = self.segments.len();
            for i in 0..num_segments {
                self.update_segment(i);
            }

            self.update_tail();
        }
    }

    fn update_segment(&mut self, segment: usize) {
        let dist = self.segments[segment].diff(&self.get_prev_segment_pos(segment));

        let vector = match dist {
            DistanceVector {
                row_diff: 2,
                column_diff: 0,
            } => DistanceVector::new(1, 0),
            DistanceVector {
                row_diff: -2,
                column_diff: 0,
            } => DistanceVector::new(-1, 0),
            DistanceVector {
                row_diff: 0,
                column_diff: 2,
            } => DistanceVector::new(0, 1),
            DistanceVector {
                row_diff: 0,
                column_diff: -2,
            } => DistanceVector::new(0, -1),
            _ => {
                if dist.euclidean_distance() > 1.5000001 {
                    dist.to_diagonal_move()
                } else {
                    DistanceVector::new(0, 0)
                }
            }
        };

        self.segments[segment] = self.segments[segment].moved(&vector);
    }

    fn get_prev_segment_pos(&self, segment: usize) -> Position {
        if segment == 0 {
            self.head.clone()
        } else {
            self.segments[segment - 1].clone()
        }
    }

    fn get_last_segment_pos(&self) -> Position {
        if !self.segments.is_empty() {
            self.segments.last().unwrap().clone()
        } else {
            self.head.clone()
        }
    }

    fn update_tail(&mut self) {
        let dist = self.tail.diff(&self.get_last_segment_pos());

        let vector = match dist {
            DistanceVector {
                row_diff: 2,
                column_diff: 0,
            } => DistanceVector::new(1, 0),
            DistanceVector {
                row_diff: -2,
                column_diff: 0,
            } => DistanceVector::new(-1, 0),
            DistanceVector {
                row_diff: 0,
                column_diff: 2,
            } => DistanceVector::new(0, 1),
            DistanceVector {
                row_diff: 0,
                column_diff: -2,
            } => DistanceVector::new(0, -1),
            _ => {
                if dist.euclidean_distance() > 1.5000001 {
                    dist.to_diagonal_move()
                } else {
                    DistanceVector::new(0, 0)
                }
            }
        };

        self.tail = self.tail.moved(&vector);

        self.known_tail_locations.insert(self.tail.clone());
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str(s: &str) -> Direction {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    distance: usize,
}

impl Move {
    fn from_str(line: &str) -> Move {
        let mut parts = line.split(' ');

        let direction = Direction::from_str(parts.next().unwrap());
        let distance = parts.next().unwrap().parse().unwrap();

        Move {
            direction,
            distance,
        }
    }

    fn to_distance_vector(&self) -> DistanceVector {
        match self.direction {
            Direction::Up => DistanceVector::new(1, 0),
            Direction::Down => DistanceVector::new(-1, 0),
            Direction::Left => DistanceVector::new(0, -1),
            Direction::Right => DistanceVector::new(0, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::fs::File;

    #[test]
    fn test_part_1_example() {
        let f = File::open("inputs/nine_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 13;
        let actual = count_tail_visits(&values, 0);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_1_actual() {
        let f = File::open("inputs/nine.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 6498;
        let actual = count_tail_visits(&values, 0);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_2_example() {
        let f = File::open("inputs/nine_example_2.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 36;
        let actual = count_tail_visits(&values, 8);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_2_actual() {
        let f = File::open("inputs/nine.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 2531;
        let actual = count_tail_visits(&values, 8);

        assert_eq!(expected, actual)
    }
}
