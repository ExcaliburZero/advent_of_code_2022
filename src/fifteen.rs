use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_num_impossible_positions(&values);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_beacon_frequency(&values);

    println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> Vec<Sensor> {
    let mut sensors: Vec<Sensor> = Vec::new();
    for line in reader.lines() {
        sensors.push(Sensor::from_str(line.unwrap().as_str()));
    }

    sensors
}

#[derive(Debug)]
struct Sensor {
    sensor_position: Position,
    beacon_position: Position,
    dist: i32,
}

impl Sensor {
    fn from_str(s: &str) -> Sensor {
        let parts: Vec<&str> = s.split(": ").collect();

        let sensor_position = Position::from_str(parts[0].split(" at ").last().unwrap());
        let beacon_position = Position::from_str(parts[1].split(" at ").last().unwrap());

        let dist = sensor_position.manhatten_distance(&beacon_position);

        Sensor {
            sensor_position,
            beacon_position,
            dist,
        }
    }

    /*
        fn calc_boundary_positions(&self) -> Vec<Position> {
            let mut positions = vec![];
            let d = self.dist + 1;
            for row in self.sensor_position.row - d..self.sensor_position.row + d + 1 {
                for column in self.sensor_position.column - d..self.sensor_position.column + d + 1 {
                    let position = Position::new(row, column);

                    positions.push(position);
                }
            }

            positions
        }
    */

    fn calc_extent_lines(&self) -> Vec<Line> {
        let top = self.sensor_position.moved(-(self.dist + 1), 0);
        let bottom = self.sensor_position.moved(self.dist + 1, 0);

        let mut lines = vec![];
        for position in vec![top, bottom] {
            for slope in &[-1, 1] {
                let line = Line::from_position_and_slope(&position, *slope);

                lines.push(line);
            }
        }

        lines
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
        let parts: Vec<&str> = s.split(", ").collect();

        Position {
            row: parts[1].split('=').last().unwrap().parse().unwrap(),
            column: parts[0].split('=').last().unwrap().parse().unwrap(),
        }
    }

    fn moved(&self, row_change: i32, column_change: i32) -> Position {
        let new_row = row_change + self.row as i32;
        let new_column = column_change + self.column as i32;

        Position::new(new_row, new_column)
    }

    fn manhatten_distance(&self, other: &Position) -> i32 {
        (other.row - self.row).abs() + (other.column - self.column).abs()
    }
}

/*
#[derive(Clone, Copy, Eq, PartialEq)]
enum CellState {
    Beacon, Sensor, KnownEmpty, Unknown
}

struct Grid {
    cells: HashMap<Position, CellState>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            cells: HashMap::new(),
        }
    }

    fn set(&mut self, position: &Position, value: CellState) {
        self.cells.insert(position.clone(), value);
    }

    fn set_if_empty(&mut self, position: &Position, value: CellState) {
        if !self.cells.contains_key(position) {
            self.cells.insert(position.clone(), value);
        }
    }

    fn fill_blanks_within_distance(&mut self, source: &Position, manhatten_distance: i32, value: CellState) {
        let mut visited: HashSet<Position> = HashSet::new();
        visited.insert(source.clone());

        let mut to_visit = source.get_neighbors();
        while !to_visit.is_empty() {
            let current = to_visit.remove(0);
            //println!("{current:?}");

            if visited.contains(&current) || source.manhatten_distance(&current) > manhatten_distance {
                continue;
            }
            //println!("new {current:?}");

            visited.insert(current.clone());

            self.set_if_empty(&current, value);
            for neighbor in current.get_neighbors() {
                if visited.contains(&neighbor) || source.manhatten_distance(&neighbor) > manhatten_distance {
                } else {
                    to_visit.push(neighbor);
                }
            }
        }
    }

    fn get(&self, position: &Position) -> CellState {
        self.cells.get(position).cloned().unwrap_or(CellState::Unknown)
    }

    fn print(&self, top_left: &Position, bottom_right: &Position) {
        for row in top_left.row..bottom_right.row + 1 {
            for column in top_left.column..bottom_right.column + 1 {
                print!("{}", match self.get(&Position::new(row, column)) {
                    CellState::Unknown => '.',
                    CellState::Beacon => 'B',
                    CellState::Sensor => 'S',
                    CellState::KnownEmpty => '#',
                });
            }
            println!("");
        }
    }
}
*/

fn would_be_visible(position: &Position, sensors: &[Sensor]) -> bool {
    for sensor in sensors.iter() {
        if sensor.sensor_position.manhatten_distance(position) <= sensor.dist
            || *position == sensor.beacon_position
        {
            return true;
        }
    }

    false
}

fn find_num_impossible_positions(sensors: &[Sensor]) -> i32 {
    let search_row = 2000000; //10;
    let mut min_col_extent = None;
    let mut max_col_extent = None;
    for sensor in sensors.iter() {
        let min_column = sensor.sensor_position.column - sensor.dist;
        let max_column = sensor.sensor_position.column + sensor.dist;

        if min_col_extent.is_none() {
            min_col_extent = Some(min_column);
        }
        if max_col_extent.is_none() {
            max_col_extent = Some(max_column);
        }

        if min_col_extent.unwrap() > min_column {
            min_col_extent = Some(min_column);
        }
        if max_col_extent.unwrap() < max_column {
            max_col_extent = Some(max_column);
        }
    }

    let min_col_extent = min_col_extent.unwrap();
    let max_col_extent = max_col_extent.unwrap();

    let mut total = 0;
    for column in min_col_extent..max_col_extent + 1 {
        let position = Position::new(search_row, column);

        if would_be_visible(&position, sensors) {
            total += 1;
        }
    }

    total
}

#[derive(Clone, Debug)]
struct Line {
    slope: i32,
    y_intercept: i32,
}

impl Line {
    fn new(slope: i32, y_intercept: i32) -> Line {
        Line { slope, y_intercept }
    }

    fn from_position_and_slope(position: &Position, slope: i32) -> Line {
        let y_intercept = position.row + position.column * -slope;

        Line::new(slope, y_intercept)
    }

    fn intersection(&self, other: &Line) -> Option<Position> {
        if self.slope != other.slope {
            let x = (other.y_intercept - self.y_intercept) / (self.slope - other.slope);
            let y = self.slope * x + self.y_intercept;
            return Some(Position::new(y, x));
        }

        // Could be either same line or parallel
        None
    }
}

fn find_beacon_frequency(sensors: &[Sensor]) -> i64 {
    //let range = 0..20 + 1;
    let range = 0..4000000 + 1;

    let lines: Vec<Line> = sensors.iter().flat_map(|s| s.calc_extent_lines()).collect();

    let possibilities: Vec<Position> = lines
        .iter()
        .cloned()
        .flat_map(|l1| lines.iter().flat_map(move |l2| l1.intersection(l2)))
        .collect();

    let mut hidden_beacon = None;
    for position in possibilities.iter() {
        if !range.contains(&position.column) || !range.contains(&position.row) {
            continue;
        }

        if !would_be_visible(position, sensors) {
            hidden_beacon = Some(position.clone());
        }
    }

    let hidden_beacon = hidden_beacon.unwrap();
    hidden_beacon.column as i64 * 4000000 + hidden_beacon.row as i64
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
