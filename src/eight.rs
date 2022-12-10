use std::collections::HashSet;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = count_visible_trees(&values);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_best_senic_score(&values);

    println!("{}", answer);
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn new(r: usize, c: usize) -> Position {
        Position { row: r, column: c }
    }

    fn moved(&self, row_change: i32, column_change: i32) -> Option<Position> {
        let new_row = row_change + self.row as i32;
        let new_column = column_change + self.column as i32;

        if new_row < 0 || new_column < 0 {
            return None;
        }

        Some(Position::new(new_row as usize, new_column as usize))
    }
}

struct Grid {
    trees: Vec<Vec<i32>>,
}

impl Grid {
    fn from_lines(lines: &[String]) -> Grid {
        let mut trees = vec![vec![0; lines.get(0).unwrap().len()]; lines.len()];

        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                trees[i][j] = c.to_string().parse().unwrap();
            }
        }

        Grid { trees }
    }

    fn get_height(&self) -> usize {
        self.trees.len()
    }

    fn get_width(&self) -> usize {
        self.trees.get(1).unwrap().len()
    }

    fn contains(&self, pos: &Position) -> bool {
        // Note: these first two are implied by usize
        // pos.row >= 0
        // && pos.column >= 0
        pos.row < self.get_height() && pos.column < self.get_width()
    }

    fn get_visible_trees(&self) -> Vec<Position> {
        let mut visibile = VisibilityMap::new();

        for r in 0..self.get_height() {
            for c in 0..self.get_width() {
                let pos = Position::new(r, c);

                // Mark edges
                if r == 0 || c == 0 || r == self.get_height() - 1 || c == self.get_width() - 1 {
                    visibile.mark_visible(&pos);
                    continue;
                }

                if visibile.is_known_visible(&pos) {
                    continue;
                }

                for direction in vec![
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ] {
                    let start = match direction {
                        Direction::Up => Position::new(self.get_height() - 1, c),
                        Direction::Down => Position::new(0, c),
                        Direction::Left => Position::new(r, self.get_width() - 1),
                        Direction::Right => Position::new(r, 0),
                    };

                    let mut p2 = start;
                    let mut prev_height = -9999;
                    loop {
                        let height = self.get(&p2);
                        if height > prev_height {
                            if p2 == pos {
                                visibile.mark_visible(&pos);
                                break;
                            } else {
                                prev_height = height;
                            }
                        }

                        if p2 == pos {
                            break;
                        }

                        let new_p2 = direction.advance(&p2);
                        if let Some(new_p2) = new_p2 {
                            if self.contains(&new_p2) {
                                p2 = new_p2;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        visibile.known_visibile.iter().cloned().collect()
    }

    fn get(&self, pos: &Position) -> i32 {
        self.trees[pos.row][pos.column]
    }

    fn calc_senic_score(&self, pos: &Position) -> i32 {
        let mut score = 1;
        let source_height = self.get(pos);

        for direction in vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let mut p2 = pos.clone();
            let mut count = 1;
            loop {
                let new_p2 = direction.advance(&p2);

                if new_p2.is_none() {
                    count -= 1;
                    break;
                }

                let new_p2 = new_p2.unwrap();
                if !self.contains(&new_p2) {
                    count -= 1;
                    break;
                }

                let new_height = self.get(&new_p2);
                if new_height >= source_height {
                    break;
                }

                count += 1;
                p2 = new_p2;
            }
            score *= count;
        }

        score
    }

    fn find_best_senic_score(&self) -> i32 {
        let mut max_senic_score = 0;

        for r in 0..self.get_height() {
            for c in 0..self.get_width() {
                let pos = Position::new(r, c);

                let score = self.calc_senic_score(&pos);
                if score > max_senic_score {
                    max_senic_score = score;
                }
            }
        }

        max_senic_score
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn advance(&self, pos: &Position) -> Option<Position> {
        match self {
            Direction::Up => pos.moved(-1, 0),
            Direction::Down => pos.moved(1, 0),
            Direction::Left => pos.moved(0, -1),
            Direction::Right => pos.moved(0, 1),
        }
    }
}

struct VisibilityMap {
    known_visibile: HashSet<Position>,
}

impl VisibilityMap {
    fn new() -> VisibilityMap {
        VisibilityMap {
            known_visibile: HashSet::new(),
        }
    }

    fn mark_visible(&mut self, position: &Position) {
        self.known_visibile.insert(position.clone());
    }

    fn is_known_visible(&self, position: &Position) -> bool {
        self.known_visibile.contains(position)
    }
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> Grid {
    Grid::from_lines(&reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>())
}

fn count_visible_trees(grid: &Grid) -> i32 {
    grid.get_visible_trees().len() as i32
}

fn find_best_senic_score(grid: &Grid) -> i32 {
    grid.find_best_senic_score()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::fs::File;

    #[test]
    fn test_part_1_example() {
        let f = File::open("inputs/eight_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 21;
        let actual = count_visible_trees(&values);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_1_actual() {
        let f = File::open("inputs/eight.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 1870;
        let actual = count_visible_trees(&values);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_2_example() {
        let f = File::open("inputs/eight_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 8;
        let actual = find_best_senic_score(&values);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_2_actual() {
        let f = File::open("inputs/eight.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 517440;
        let actual = find_best_senic_score(&values);

        assert_eq!(expected, actual)
    }
}
