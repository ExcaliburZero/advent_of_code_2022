use std::collections::{BTreeMap, HashMap};
use std::io::prelude::*;
use std::io::{self, BufReader};

type Direction = Position;

const NORTH: Direction = Position::new(-1, 0);
const SOUTH: Direction = Position::new(1, 0);
const EAST: Direction = Position::new(0, 1);
const WEST: Direction = Position::new(0, -1);
const NORTH_EAST: Direction = Position::new(-1, 1);
const NORTH_WEST: Direction = Position::new(-1, -1);
const SOUTH_EAST: Direction = Position::new(1, 1);
const SOUTH_WEST: Direction = Position::new(1, -1);

const NORTH_RULE: MoveRule = MoveRule::new([NORTH, NORTH_EAST, NORTH_WEST], &NORTH);
const SOUTH_RULE: MoveRule = MoveRule::new([SOUTH, SOUTH_EAST, SOUTH_WEST], &SOUTH);
const EAST_RULE: MoveRule = MoveRule::new([EAST, NORTH_EAST, SOUTH_EAST], &EAST);
const WEST_RULE: MoveRule = MoveRule::new([WEST, NORTH_WEST, SOUTH_WEST], &WEST);

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_num_empty_after_rounds(&values, 10);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    //let answer = find_start_marker_2(&values[0]);

    //println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> State {
    State::from_lines(&reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>())
}

fn find_num_empty_after_rounds(initial_state: &State, num_rounds: i32) -> i32 {
    let mut state = initial_state.clone();
    //state.print();
    //println!("===============================================================");
    for round in 0..num_rounds {
        state = state.advance(round);

        //state.print();
        //println!("===============================================================");
    }

    let bounds = state.get_bounds();

    bounds.num_tiles() - state.elf_positions.len() as i32
}

type ElfId = i32;

#[derive(Debug, Clone)]
struct Bounds2D {
    top_left: Position,
    bottom_right: Position,
}

impl Bounds2D {
    fn from_positions(positions: &[Position]) -> Bounds2D {
        let top = positions.iter().map(|pos| pos.row).min().unwrap();
        let bottom = positions.iter().map(|pos| pos.row).max().unwrap();
        let left = positions.iter().map(|pos| pos.column).min().unwrap();
        let right = positions.iter().map(|pos| pos.column).max().unwrap();

        let top_left = Position::new(top, left);
        let bottom_right = Position::new(bottom, right);

        Bounds2D {
            top_left,
            bottom_right,
        }
    }

    fn num_tiles(&self) -> i32 {
        let num_rows = self.bottom_right.row + 1 - self.top_left.row;
        let num_columns = self.bottom_right.column + 1 - self.top_left.column;

        num_rows * num_columns
    }

    fn get_positions_tl_to_br(&self) -> Vec<Position> {
        let mut positions = vec![];
        for row in self.top_left.row..=self.bottom_right.row {
            for column in self.top_left.column..=self.bottom_right.column {
                positions.push(Position::new(row, column));
            }
        }

        positions
    }
}

#[derive(Debug, Clone)]
struct State {
    elf_positions: BTreeMap<ElfId, Position>,
}

impl State {
    fn new(elf_positions: BTreeMap<ElfId, Position>) -> State {
        State { elf_positions }
    }

    fn print(&self) {
        let bounds = self.get_bounds();

        let mut last_row = None;
        for position in bounds.get_positions_tl_to_br() {
            if let Some(prev_row) = last_row {
                if prev_row < position.row {
                    println!();
                    print!("{:3} ", position.row);
                }
            } else {
                print!("{:3} ", position.row);
            }

            if self.elf_positions.values().any(|p| *p == position) {
                print!("#");
            } else {
                print!(".");
            }

            last_row = Some(position.row);
        }

        println!();
    }

    fn from_lines(lines: &[String]) -> State {
        let mut elf_positions: BTreeMap<ElfId, Position> = BTreeMap::new();
        let mut current_id = 0;

        for (row, line) in lines.iter().enumerate() {
            for (column, c) in line.chars().enumerate() {
                if c == '#' {
                    let position = Position::new(row as i32, column as i32);
                    elf_positions.insert(current_id, position);

                    current_id += 1;
                }
            }
        }

        State::new(elf_positions)
    }

    fn has_elf(&self, position: &Position) -> bool {
        self.elf_positions.values().any(|p| p == position)
    }

    fn advance(&self, round: i32) -> State {
        let rules = State::get_rules(round);
        let mut proposed_elf_positions: HashMap<Position, Vec<(ElfId, Position)>> = HashMap::new();

        for (elf_id, position) in self.elf_positions.iter() {
            let all_neighbors_empty = position.get_neighbors().iter().all(|n| !self.has_elf(n));

            let mut direction = Position::new(0, 0);
            if !all_neighbors_empty {
                for rule in rules.iter() {
                    if rule.can_apply(self, position) {
                        direction = rule.destination;
                        break;
                    }
                }
            }

            let new_position = position.moved(&direction);
            proposed_elf_positions
                .entry(new_position)
                .or_default()
                .push((*elf_id, *position));
        }

        let mut new_elf_positions: BTreeMap<i32, Position> = BTreeMap::new();
        for (new_position, elves) in proposed_elf_positions.iter() {
            if elves.len() == 1 {
                let (elf_id, _) = elves[0];
                new_elf_positions.insert(elf_id, *new_position);
                continue;
            }

            for (elf_id, old_position) in elves.iter() {
                new_elf_positions.insert(*elf_id, *old_position);
            }
        }

        assert_eq!(self.elf_positions.len(), new_elf_positions.len());

        State::new(new_elf_positions)
    }

    fn get_rules(round: i32) -> Vec<MoveRule> {
        let mut rules = vec![NORTH_RULE, SOUTH_RULE, WEST_RULE, EAST_RULE];
        rules.rotate_left((round % 4) as usize);

        rules
    }

    fn get_bounds(&self) -> Bounds2D {
        Bounds2D::from_positions(
            &self
                .elf_positions
                .values()
                .copied()
                .collect::<Vec<Position>>(),
        )
    }
}

#[derive(Clone, Debug)]
struct MoveRule {
    check_all_empty: [Direction; 3],
    destination: Direction,
}

impl MoveRule {
    const fn new(check_all_empty: [Direction; 3], destination: &Position) -> MoveRule {
        let destination = *destination;

        MoveRule {
            check_all_empty,
            destination,
        }
    }

    fn can_apply(&self, state: &State, position: &Direction) -> bool {
        for direction in self.check_all_empty.iter() {
            let neighbor = position.moved(direction);
            if state.has_elf(&neighbor) {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Position {
    row: i32,
    column: i32,
}

impl Position {
    const fn new(row: i32, column: i32) -> Position {
        Position { row, column }
    }

    fn moved(&self, other: &Position) -> Position {
        Position::new(self.row + other.row, self.column + other.column)
    }

    fn get_neighbors(&self) -> Vec<Position> {
        vec![
            NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST, NORTH_WEST,
        ]
        .iter()
        .map(|direction| self.moved(direction))
        .collect()
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
