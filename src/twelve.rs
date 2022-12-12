use std::collections::HashSet;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_shortest_path(&values);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_shortest_path_2(&values);

    println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> Graph {
    Graph::from_lines(&reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>())
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

    fn moved(&self, row_diff: i32, column_diff: i32) -> Position {
        Position::new(self.row + row_diff, self.column + column_diff)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Path {
    tiles: Vec<Position>,
}

impl Path {
    fn new() -> Path {
        Path { tiles: vec![] }
    }

    fn appended(&self, position: &Position) -> Path {
        let mut new_path = self.clone();
        new_path.tiles.push(position.clone());

        new_path
    }
}

struct Graph {
    tiles: Vec<Vec<i32>>,
    start: Position,
    end: Position,
}

impl Graph {
    fn from_lines(lines: &[String]) -> Graph {
        let mut tiles = vec![];
        let mut start = None;
        let mut end = None;
        for (r, line) in lines.iter().enumerate() {
            let mut row = vec![];
            for (c, char) in line.chars().enumerate() {
                if char == 'S' {
                    start = Some(Position::new(r as i32, c as i32));
                    row.push(0);
                } else if char == 'E' {
                    end = Some(Position::new(r as i32, c as i32));
                    row.push(25);
                } else {
                    row.push(char as i32 - 'a' as i32);
                }
            }

            tiles.push(row);
        }

        let start = start.unwrap();
        let end = end.unwrap();

        Graph { tiles, start, end }
    }

    fn get_height(&self) -> usize {
        self.tiles.len()
    }

    fn get_width(&self) -> usize {
        self.tiles[0].len()
    }

    fn is_in_bounds(&self, position: &Position) -> bool {
        position.row >= 0
            && position.column >= 0
            && position.row < self.get_height() as i32
            && position.column < self.get_width() as i32
    }

    fn get_neighbors(&self, position: &Position) -> Vec<Position> {
        vec![
            position.moved(1, 0),
            position.moved(-1, 0),
            position.moved(0, 1),
            position.moved(0, -1),
        ]
        .iter()
        .filter(|p| self.is_in_bounds(p))
        .cloned()
        .collect()
    }

    fn bfs(&self, source: &Position, destination: &Position) -> Option<i32> {
        let mut visited: HashSet<Position> = HashSet::new();
        visited.insert(source.clone());

        let mut to_visit = vec![];
        for neighbor in self.get_neighbors(source) {
            if self.is_stepable(source, &neighbor) {
                to_visit.push((neighbor.clone(), Path::new().appended(&neighbor)));
            }
        }

        while !to_visit.is_empty() {
            let (current, cur_path) = to_visit[0].clone();
            to_visit.remove(0);

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            if current == *destination {
                return Some(cur_path.tiles.len() as i32);
            }

            for neighbor in self.get_neighbors(&current) {
                if !visited.contains(&neighbor) && self.is_stepable(&current, &neighbor) {
                    to_visit.push((neighbor.clone(), cur_path.appended(&neighbor)));
                }
            }
        }

        None
    }

    fn get_value(&self, position: &Position) -> i32 {
        self.tiles[position.row as usize][position.column as usize]
    }

    fn is_stepable(&self, src: &Position, dest: &Position) -> bool {
        let src_h = self.get_value(src);
        let dest_h = self.get_value(dest);

        src_h + 1 >= dest_h
    }

    fn get_positions_with_height(&self, height: i32) -> Vec<Position> {
        let mut matches = vec![];
        for row in 0..self.get_height() {
            for column in 0..self.get_width() {
                let position = Position::new(row as i32, column as i32);
                if self.get_value(&position) == height {
                    matches.push(position)
                }
            }
        }

        matches
    }
}

fn find_shortest_path(graph: &Graph) -> i32 {
    graph.bfs(&graph.start, &graph.end).unwrap()
}

fn find_shortest_path_2(graph: &Graph) -> i32 {
    graph
        .get_positions_with_height(0)
        .iter()
        .filter_map(|pos| graph.bfs(pos, &graph.end))
        .min()
        .unwrap()
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
