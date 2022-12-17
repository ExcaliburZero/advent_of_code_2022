use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_tower_height(&values, 2022);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_tower_height(&values, 1000000000000);

    println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> JetPattern {
    let line = reader.lines().next().unwrap().unwrap();

    JetPattern::from_str(&line)
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Fingerprint {
    pattern_modulo: i64,
    direction_modulo: i64,
    tiles_pattern: BTreeSet<Position>,
}

impl Fingerprint {
    fn new(grid: &Grid, j: i64, jet_pattern: &JetPattern) -> Fingerprint {
        Fingerprint {
            pattern_modulo: j % 5,
            direction_modulo: j % jet_pattern.directions.len() as i64,
            tiles_pattern: grid.get_hashable_chunk(),
        }
    }
}

#[derive(Debug, Clone)]
struct Occurance {
    i: i64,
    j: i64,
    height: i64,
}

fn find_tower_height(jet_pattern: &JetPattern, num_rocks: i64) -> i64 {
    let mut grid = Grid::new();

    let mut fingerprints: HashMap<Fingerprint, Vec<Occurance>> = HashMap::new();

    let mut calculated_offset = 0;

    let mut j = 0;
    let mut i = -1;
    while i < num_rocks - 1 {
        // Weird check so that we can adjust i as needed in loop
        i += 1; // maybe less error prone than making sure to increment at end
        let mut position = Position::new(grid.get_heighest_row() + 4, 4);
        let rock_pattern = RockPattern::from_round_number(i);

        //grid.print(&rock_pattern, &position);
        //println!("");

        loop {
            let direction = jet_pattern.get_direction(j);
            let position_lr = position.moved(0, direction.to_column_offset());

            if grid.hits_anything(&rock_pattern, &position_lr).is_none() {
                position = position_lr;
            }

            let position_down = position.moved(-1, 0);
            if grid.hits_anything(&rock_pattern, &position_down).is_some() {
                for p in rock_pattern.all_positions(&position) {
                    grid.set(&p);
                }
                j += 1;
                break;
            }

            position = position_down;

            //grid.print(&rock_pattern, &position);
            //println!("");

            j += 1;
        }

        let fingerprint = Fingerprint::new(&grid, j, jet_pattern);
        let height = grid.get_heighest_row();
        let occurance = Occurance { i, j, height };
        if fingerprints.contains_key(&fingerprint) {
            let prev_occurance = fingerprints.get(&fingerprint).unwrap()[0].clone();

            let i_diff = occurance.i - prev_occurance.i;
            let j_diff = occurance.j - prev_occurance.j;
            let height_diff = occurance.height - prev_occurance.height;

            let remaining_rocks = (num_rocks - 1) - i;
            let num_remaining_recurrances = remaining_rocks / i_diff;

            i += num_remaining_recurrances * i_diff;
            j += num_remaining_recurrances * j_diff;

            calculated_offset += height_diff * num_remaining_recurrances;
        } else {
            fingerprints.insert(fingerprint.clone(), vec![]);
        }

        fingerprints.get_mut(&fingerprint).unwrap().push(occurance);
    }

    grid.get_heighest_row() + calculated_offset
}

struct RockPattern {
    relative_rocks: HashSet<Position>,
}

impl RockPattern {
    fn all_positions(&self, reference: &Position) -> Vec<Position> {
        self.relative_rocks
            .iter()
            .map(|relative| reference.moved(relative.row, relative.column))
            .collect()
    }

    fn shape_minus() -> RockPattern {
        RockPattern {
            relative_rocks: [
                Position::new(0, -1),
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 2),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    fn shape_plus() -> RockPattern {
        RockPattern {
            relative_rocks: [
                Position::new(1, -1),
                Position::new(1, 0),
                Position::new(0, 0),
                Position::new(2, 0),
                Position::new(1, 1),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    fn shape_missngno() -> RockPattern {
        RockPattern {
            relative_rocks: [
                Position::new(0, -1),
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(1, 1),
                Position::new(2, 1),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    fn shape_vertical_bar() -> RockPattern {
        RockPattern {
            relative_rocks: [
                Position::new(0, -1),
                Position::new(1, -1),
                Position::new(2, -1),
                Position::new(3, -1),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    fn shape_square() -> RockPattern {
        RockPattern {
            relative_rocks: [
                Position::new(0, -1),
                Position::new(1, -1),
                Position::new(0, 0),
                Position::new(1, 0),
            ]
            .iter()
            .cloned()
            .collect(),
        }
    }

    fn from_round_number(round_number: i64) -> RockPattern {
        match round_number % 5 {
            0 => RockPattern::shape_minus(),
            1 => RockPattern::shape_plus(),
            2 => RockPattern::shape_missngno(),
            3 => RockPattern::shape_vertical_bar(),
            4 => RockPattern::shape_square(),
            _ => panic!(),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct JetPattern {
    directions: Vec<Direction>,
}

impl JetPattern {
    fn from_str(s: &str) -> JetPattern {
        JetPattern {
            directions: s.chars().map(Direction::from_char).collect(),
        }
    }

    fn get_direction(&self, step: i64) -> Direction {
        self.directions[step as usize % self.directions.len()]
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!(),
        }
    }

    fn to_column_offset(self) -> i64 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Ord, PartialOrd)]
struct Position {
    row: i64,
    column: i64,
}

impl Position {
    fn new(r: i64, c: i64) -> Position {
        Position { row: r, column: c }
    }

    fn moved(&self, row_change: i64, column_change: i64) -> Position {
        let new_row = row_change + self.row as i64;
        let new_column = column_change + self.column as i64;

        Position::new(new_row, new_column)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum HitType {
    Wall,
    Floor,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum TileState {
    Empty,
    Filled,
}

struct Grid {
    tiles: HashSet<Position>,
    bottom_row: i64,
    left_wall: i64,
    right_wall: i64,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            tiles: HashSet::new(),
            bottom_row: 0,
            left_wall: 0,  // index 0
            right_wall: 8, // index 8
        }
    }

    fn get(&self, position: &Position) -> TileState {
        if self.tiles.contains(position) {
            TileState::Filled
        } else {
            TileState::Empty
        }
    }

    fn set(&mut self, position: &Position) {
        self.tiles.insert(position.clone());
    }

    fn is_in_wall(&self, position: &Position) -> bool {
        position.column <= self.left_wall || position.column >= self.right_wall
    }

    fn is_in_floor(&self, position: &Position) -> bool {
        position.row <= self.bottom_row
    }

    fn get_heighest_row(&self) -> i64 {
        self.tiles.iter().map(|p| p.row).max().unwrap_or(0)
    }

    fn hits_anything(&self, rock_pattern: &RockPattern, position: &Position) -> Option<HitType> {
        let positions = rock_pattern.all_positions(position);

        let hit_types: HashSet<HitType> = positions
            .iter()
            .flat_map(|p| {
                if self.is_in_floor(p) || self.get(p) == TileState::Filled {
                    Some(HitType::Floor)
                } else if self.is_in_wall(p) {
                    Some(HitType::Wall)
                } else {
                    None
                }
            })
            .collect();

        if hit_types.contains(&HitType::Floor) {
            Some(HitType::Floor)
        } else if hit_types.contains(&HitType::Wall) {
            Some(HitType::Wall)
        } else {
            None
        }
    }

    //fn top_row_is_tetris(&self) -> bool {
    /*fn any_row_is_tetris(&self) -> bool {
        let top_row = self.get_heighest_row();

        for row in 1..top_row + 1 {
            let mut any_empty = false;
            for column in self.left_wall + 1..self.right_wall {
                let position = Position::new(row, column);
                if self.get(&position) == TileState::Empty {
                    any_empty = true;
                }
            }

            if !any_empty {
                return true;
            }
        }

        false
    }*/

    fn get_hashable_chunk(&self) -> BTreeSet<Position> {
        let highest_row = self.get_heighest_row();

        let mut lower_row = highest_row;
        for column in self.left_wall + 1..self.right_wall {
            let mut row = highest_row;
            loop {
                let position = Position::new(row, column);
                if row == self.bottom_row || self.get(&position) == TileState::Filled {
                    break;
                }

                row -= 1;
            }
            row += 1;
            lower_row = *[lower_row, row].iter().min().unwrap();
        }

        self.tiles
            .iter()
            .filter(|p| p.row > lower_row)
            .map(|p| p.moved(-lower_row, 0))
            .collect()
    }

    /*fn print(&self, rock_pattern: &RockPattern, position: &Position) {
        let all_pattern_positions = rock_pattern.all_positions(position);

        for row in (self.bottom_row + 1..self.get_heighest_row() + 3 + 4 + 1).rev() {
            print!("|");
            for column in self.left_wall + 1..self.right_wall {
                let p = Position::new(row, column);
                print!(
                    "{}",
                    match self.tiles.get(&p) {
                        Some(_) => '#',
                        None => {
                            if all_pattern_positions.contains(&p) {
                                '@'
                            } else {
                                '.'
                            }
                        }
                    }
                );
            }
            println!("|");
        }

        for _ in self.left_wall..self.right_wall + 1 {
            print!("-");
        }
        println!();
    }*/
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::fs::File;

    #[test]
    fn test_part_1_example() {
        let f = File::open("inputs/seventeen_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 3068;
        let actual = find_tower_height(&values, 2022);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_1_actual() {
        let f = File::open("inputs/seventeen.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 3177;
        let actual = find_tower_height(&values, 2022);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_2_example() {
        let f = File::open("inputs/seventeen_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 1514285714288;
        let actual = find_tower_height(&values, 1000000000000);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_2_actual() {
        let f = File::open("inputs/seventeen.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 1565517241382;
        let actual = find_tower_height(&values, 1000000000000);

        assert_eq!(expected, actual)
    }
}
