use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let mut grid = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_first_fall_sand(&mut grid);

    println!("{}", answer);
}

pub fn part_two() {
    let mut grid = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_sand_reach_source(&mut grid);

    println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> Grid {
    let mut grid = Grid::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let lines = Lines::from_str(&line);

        grid.add_lines(&lines, Tile::Rock);
    }

    grid.set_bottom();

    grid
}

fn find_sand_reach_source(grid: &mut Grid) -> i32 {
    let sand_source = Position::from_str("500,0");

    grid.override_bottom(grid.tiles.keys().map(|p| p.row).max().unwrap() + 2);

    let mut current_sand = 1;
    loop {
        grid.set(&sand_source, Tile::Sand);

        let mut sand_position = sand_source.clone();
        loop {
            let result = grid.step(&sand_position, true);
            match result {
                Some(p2) => sand_position = p2,
                None => {
                    if sand_position == sand_source {
                        return current_sand;
                    }

                    break;
                }
            }
        }

        current_sand += 1;
    }
}

fn find_first_fall_sand(grid: &mut Grid) -> i32 {
    let sand_source = Position::from_str("500,0");

    let mut current_sand = 1;
    loop {
        grid.set(&sand_source, Tile::Sand);

        let mut sand_position = sand_source.clone();
        loop {
            let result = grid.step(&sand_position, false);
            match result {
                Some(p2) => sand_position = p2,
                None => break, // sand stopped moving
            }

            if sand_position.row > grid.bottom.unwrap() {
                return current_sand - 1;
            }
        }

        current_sand += 1;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Rock,
    Sand,
}

#[derive(Clone)]
struct Bounds {
    top_left: Position,
    bottom_right: Position,
}

impl Bounds {
    fn from_position(position: &Position) -> Bounds {
        Bounds {
            top_left: position.clone(),
            bottom_right: position.clone(),
        }
    }

    fn update(&mut self, position: &Position) {
        if position.row < self.top_left.row {
            self.top_left = self.top_left.changed_row(position.row);
        }

        if position.column < self.top_left.column {
            self.top_left = self.top_left.changed_column(position.column);
        }

        if position.row > self.bottom_right.row {
            self.bottom_right = self.bottom_right.changed_row(position.row);
        }

        if position.column > self.bottom_right.column {
            self.bottom_right = self.bottom_right.changed_column(position.column);
        }
    }
}

struct Grid {
    tiles: HashMap<Position, Tile>,
    bounds: Option<Bounds>,
    bottom: Option<i32>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            tiles: HashMap::new(),
            bounds: None,
            bottom: None,
        }
    }

    fn step(&mut self, current_sand: &Position, has_floor: bool) -> Option<Position> {
        let down = current_sand.moved(1, 0);
        if self.get(&down, has_floor) == Tile::Empty {
            self.set(current_sand, Tile::Empty);
            self.set(&down, Tile::Sand);
            return Some(down);
        }

        let down_left = current_sand.moved(1, -1);
        if self.get(&down_left, has_floor) == Tile::Empty {
            self.set(current_sand, Tile::Empty);
            self.set(&down_left, Tile::Sand);
            return Some(down_left);
        }

        let down_right = current_sand.moved(1, 1);
        if self.get(&down_right, has_floor) == Tile::Empty {
            self.set(current_sand, Tile::Empty);
            self.set(&down_right, Tile::Sand);
            return Some(down_right);
        }

        None
    }

    fn add_lines(&mut self, lines: &Lines, fill: Tile) {
        for position in lines.get_positions() {
            self.set(&position, fill);
        }
    }

    fn set(&mut self, position: &Position, value: Tile) {
        self.tiles.insert(position.clone(), value);

        if self.bounds.is_none() {
            self.bounds = Some(Bounds::from_position(position));
        }

        self.bounds.as_mut().unwrap().update(position);
    }

    fn get(&self, position: &Position, has_floor: bool) -> Tile {
        if has_floor && position.row == self.bottom.unwrap() {
            return Tile::Rock;
        }

        self.tiles.get(position).cloned().unwrap_or(Tile::Empty)
    }

    /*fn print(&self, has_floor: bool) {
        let bounds = self.bounds.clone().unwrap();
        for row in bounds.top_left.row..bounds.bottom_right.row + 1 {
            for column in bounds.top_left.column..bounds.bottom_right.column + 1 {
                print!("{}", match self.get(&Position::new(row, column), has_floor) {
                    Tile::Empty => '.',
                    Tile::Rock => '#',
                    Tile::Sand => 'O',
                });
            }
            println!("");
        }
    }*/

    fn set_bottom(&mut self) {
        self.bottom = Some(self.bounds.clone().unwrap().bottom_right.row);
    }

    fn override_bottom(&mut self, bottom: i32) {
        self.bottom = Some(bottom);
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Position {
    row: i32,
    column: i32,
}

impl Position {
    fn new(r: i32, c: i32) -> Position {
        Position { row: r, column: c }
    }

    fn from_str(s: &str) -> Position {
        let parts: Vec<&str> = s.split(',').collect();

        Position {
            row: parts[1].parse().unwrap(),
            column: parts[0].parse().unwrap(),
        }
    }

    fn changed_row(&self, row: i32) -> Position {
        Position::new(row, self.column)
    }

    fn changed_column(&self, column: i32) -> Position {
        Position::new(self.row, column)
    }

    fn moved(&self, row_change: i32, column_change: i32) -> Position {
        let new_row = row_change + self.row;
        let new_column = column_change + self.column;

        Position::new(new_row, new_column)
    }

    fn moved_by(&self, other: &Position) -> Position {
        self.moved(other.row, other.column)
    }

    fn subtracted(&self, other: &Position) -> Position {
        Position::new(self.row - other.row, self.column - other.column)
    }

    fn normalized_cardinal(&self) -> Position {
        let row = if self.row != 0 {
            self.row / self.row.abs()
        } else {
            0
        };

        let column = if self.column != 0 {
            self.column / self.column.abs()
        } else {
            0
        };

        Position::new(row, column)
    }

    fn get_points_to(&self, other: &Position) -> Vec<Position> {
        let diff = other.subtracted(self).normalized_cardinal();

        let mut points = Vec::new();

        let mut current = self.clone();
        while current != *other {
            points.push(current.clone());
            current = current.moved_by(&diff);
        }
        points.push(current);

        points
    }
}

struct Lines {
    points: Vec<Position>,
}

impl Lines {
    fn from_str(s: &str) -> Lines {
        Lines {
            points: s.split(" -> ").map(Position::from_str).collect(),
        }
    }

    fn get_positions(&self) -> Vec<Position> {
        self.points
            .iter()
            .zip(self.points.iter().skip(1))
            .flat_map(|(p1, p2)| p1.get_points_to(p2))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::fs::File;

    #[test]
    fn test_part_1_example() {
        let f = File::open("inputs/fourteen_example.txt").unwrap();
        let mut grid = read_input(&mut BufReader::new(f));

        let expected = 24;
        let actual = find_first_fall_sand(&mut grid);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_1_actual() {
        let f = File::open("inputs/fourteen.txt").unwrap();
        let mut grid = read_input(&mut BufReader::new(f));

        let expected = 618;
        let actual = find_first_fall_sand(&mut grid);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_2_example() {
        let f = File::open("inputs/fourteen_example.txt").unwrap();
        let mut grid = read_input(&mut BufReader::new(f));

        let expected = 93;
        let actual = find_sand_reach_source(&mut grid);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_2_actual() {
        let f = File::open("inputs/fourteen.txt").unwrap();
        let mut grid = read_input(&mut BufReader::new(f));

        let expected = 26358;
        let actual = find_sand_reach_source(&mut grid);

        assert_eq!(expected, actual)
    }
}
