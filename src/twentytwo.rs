use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_final_position(&values.0, &values.1);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    //let answer = find_start_marker_2(&values[0]);

    //println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> (Grid, Path) {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let grid = Grid::from_lines(&lines[0..(lines.len() - 2)]);
    let path = Path::from_str(lines.last().unwrap());

    (grid, path)
}

fn find_final_position(grid: &Grid, path: &Path) -> i32 {
    let row = grid
        .tiles
        .iter()
        .filter(|(_, t)| **t == TileType::Empty)
        .map(|(p, _)| p.row)
        .min()
        .unwrap();
    let column = grid
        .tiles
        .iter()
        .filter(|(p, t)| p.row == row && **t == TileType::Empty)
        .map(|(p, _)| p.column)
        .min()
        .unwrap();

    let position = Position::new(row, column);
    let orientation = Position::new(0, 1);

    let mut turtle = Turtle::new(&position, &orientation);

    //let mut turtle_positions = BTreeMap::new();
    //turtle_positions.insert(turtle.position, 'T');
    //grid.print(&turtle_positions);

    for m in path.movements.iter() {
        turtle = turtle.apply(grid, m);
        //turtle_positions.insert(turtle.position, 'T');
        //grid.print(&turtle_positions);
    }

    (turtle.position.row + 1) * 1000
        + (turtle.position.column + 1) * 4
        + turtle.get_orientation_value()
}

#[derive(Debug, Clone)]
struct Path {
    movements: Vec<Movement>,
}

impl Path {
    fn from_str(s: &str) -> Path {
        let mut movements = vec![];

        let mut buffer = vec![];
        for c in s.chars() {
            match c {
                'L' => {
                    if !buffer.is_empty() {
                        let movement =
                            Movement::Forward(buffer.iter().collect::<String>().parse().unwrap());
                        movements.push(movement);
                        buffer.clear();
                    }
                    movements.push(Movement::TurnLeft);
                }
                'R' => {
                    if !buffer.is_empty() {
                        let movement =
                            Movement::Forward(buffer.iter().collect::<String>().parse().unwrap());
                        movements.push(movement);
                        buffer.clear();
                    }
                    movements.push(Movement::TurnRight);
                }
                c => buffer.push(c),
            }
        }

        if !buffer.is_empty() {
            let movement = Movement::Forward(buffer.iter().collect::<String>().parse().unwrap());
            movements.push(movement);
            buffer.clear();
        }

        Path { movements }
    }
}

#[derive(Debug, Clone, Copy)]
enum Movement {
    Forward(i32),
    TurnLeft,
    TurnRight,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Position {
    row: i32,
    column: i32,
}

type Orientation = Position;

impl Position {
    fn new(row: i32, column: i32) -> Position {
        Position { row, column }
    }

    fn moved(&self, row_diff: i32, column_diff: i32) -> Position {
        Position::new(self.row + row_diff, self.column + column_diff)
    }

    fn manhatten_length(&self) -> i32 {
        self.row.abs() + self.column.abs()
    }

    fn moved_in_direction(&self, direction: &Orientation, num_steps: i32) -> Position {
        assert_eq!(direction.manhatten_length(), 1);

        self.moved(direction.row * num_steps, direction.column * num_steps)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum TileType {
    Empty,
    Wall,
}

#[derive(Debug)]
struct Grid {
    tiles: BTreeMap<Position, TileType>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(tiles: BTreeMap<Position, TileType>) -> Grid {
        let width = tiles.keys().map(|p| p.column).max().unwrap() + 1;
        let height = tiles.keys().map(|p| p.row).max().unwrap() + 1;

        Grid {
            tiles,
            width,
            height,
        }
    }

    fn print(&self, marked_positions: &BTreeMap<Position, char>) {
        for row in 0..self.height {
            for column in 0..self.width {
                let position = Position::new(row, column);
                let tile = self.get_tile(&position);

                print!(
                    "{}",
                    match tile {
                        None =>
                            if let Some(c) = marked_positions.get(&position) {
                                *c
                            } else {
                                ' '
                            },
                        Some(TileType::Empty) =>
                            if let Some(c) = marked_positions.get(&position) {
                                *c
                            } else {
                                '.'
                            },
                        Some(TileType::Wall) =>
                            if let Some(c) = marked_positions.get(&position) {
                                *c
                            } else {
                                '#'
                            },
                    }
                );
            }
            println!();
        }
    }

    fn from_lines(lines: &[String]) -> Grid {
        let mut tiles = BTreeMap::new();
        for (row, line) in lines.iter().enumerate() {
            for column in 0..line.len() {
                let c = line.chars().nth(column);
                let position = Position::new(row as i32, column as i32);

                match c {
                    None => (),
                    Some(' ') => (),
                    Some('.') => {
                        tiles.insert(position, TileType::Empty);
                    }
                    Some('#') => {
                        tiles.insert(position, TileType::Wall);
                    }
                    Some(_) => panic!(),
                }
            }
        }

        Grid::new(tiles)
    }

    fn get_tile(&self, position: &Position) -> Option<TileType> {
        self.tiles.get(position).copied()
    }

    fn move_position_with_wrapping(
        &self,
        position: &Position,
        direction: &Orientation,
        num_steps: i32,
    ) -> Position {
        let position = position.moved_in_direction(direction, num_steps);

        Position::new(
            position.row.rem_euclid(self.height),
            position.column.rem_euclid(self.width),
        )
    }

    fn advance(
        &self,
        start_position: &Position,
        direction: &Orientation,
        num_steps: i32,
    ) -> Position {
        let mut position = *start_position;
        for _ in 1..=num_steps {
            let mut candidate_position = self.move_position_with_wrapping(&position, direction, 1);
            let mut last_valid_position = position;

            let mut done = false;
            while !done {
                let tile_type = self.get_tile(&candidate_position);
                match tile_type {
                    None => {
                        candidate_position =
                            self.move_position_with_wrapping(&candidate_position, direction, 1)
                    }
                    Some(tile_type) => match tile_type {
                        TileType::Empty => {
                            last_valid_position = candidate_position;
                            done = true
                        }
                        TileType::Wall => {
                            candidate_position = last_valid_position;
                            done = true;
                        }
                    },
                }

                position = candidate_position;
            }
        }

        position
    }
}

#[derive(Debug, Clone)]
struct Turtle {
    position: Position,
    orientation: Orientation,
}

impl Turtle {
    fn new(position: &Position, orientation: &Orientation) -> Turtle {
        let position = *position;
        let orientation = *orientation;

        Turtle {
            position,
            orientation,
        }
    }

    fn get_orientation_value(&self) -> i32 {
        match self.orientation {
            Position { row: 0, column: 1 } => 0,
            Position { row: 1, column: 0 } => 1,
            Position { row: 0, column: -1 } => 2,
            Position { row: -1, column: 0 } => 3,
            _ => panic!(),
        }
    }

    fn apply(&self, grid: &Grid, movement: &Movement) -> Turtle {
        match movement {
            Movement::Forward(num_steps) => Turtle::new(
                &grid.advance(&self.position, &self.orientation, *num_steps),
                &self.orientation,
            ),
            Movement::TurnRight => self.rotated_right(),
            Movement::TurnLeft => self.rotated_right().rotated_right().rotated_right(),
        }
    }

    fn rotated_right(&self) -> Turtle {
        let new_orientation = match self.orientation {
            Position { row: 0, column: 1 } => Position::new(1, 0),
            Position { row: 1, column: 0 } => Position::new(0, -1),
            Position { row: 0, column: -1 } => Position::new(-1, 0),
            Position { row: -1, column: 0 } => Position::new(0, 1),
            _ => panic!(),
        };

        Turtle::new(&self.position, &new_orientation)
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
