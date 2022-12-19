use std::collections::HashSet;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = calc_num_sides(&values);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = calc_num_sides_no_bubbles(&values);

    println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> Droplet {
    Droplet::from_lines(&reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>())
}

fn calc_num_sides(droplet: &Droplet) -> i32 {
    let mut num_sides = 0;
    for cube in droplet.cubes.iter() {
        for neighbor in cube.get_neighbors() {
            if !droplet.cubes.contains(&neighbor) {
                num_sides += 1;
            }
        }
    }

    num_sides
}

fn calc_num_sides_no_bubbles(droplet: &Droplet) -> i32 {
    let mut num_sides_with_bubbles = 0;
    let mut maybe_bubbles = vec![];
    for cube in droplet.cubes.iter() {
        for neighbor in cube.get_neighbors() {
            if !droplet.cubes.contains(&neighbor) {
                num_sides_with_bubbles += 1;
                maybe_bubbles.push(neighbor);
            }
        }
    }

    let mut bubbles = vec![];

    let mut num_sides = num_sides_with_bubbles;
    let mut known_bubbles: HashSet<Cube> = HashSet::new();
    for cube in maybe_bubbles.iter() {
        if known_bubbles.contains(cube) {
            continue;
        }

        let maybe_bubble = droplet.get_bubble(cube);
        if let Some(bubble) = maybe_bubble {
            let bubble_sides = calc_num_sides(&bubble);
            num_sides -= bubble_sides;

            for bubble_cube in bubble.cubes.iter() {
                known_bubbles.insert(bubble_cube.clone());
            }

            bubbles.push(bubble);
        }
    }

    // Double check some properties
    for bubble in bubbles.clone().iter() {
        for cube in bubble.cubes.iter() {
            for neighbor in cube.get_neighbors() {
                if droplet.cubes.contains(cube) {
                    panic!()
                }

                for bubble_2 in bubbles.iter() {
                    if bubble == bubble_2 {
                        continue;
                    }

                    if bubble_2.cubes.contains(&neighbor) {
                        panic!();
                    }
                }
            }
        }
    }

    num_sides
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Cube {
        Cube { x, y, z }
    }

    fn from_str(s: &str) -> Cube {
        let parts: Vec<&str> = s.split(',').collect();

        Cube {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        }
    }

    fn moved(&self, x_diff: i32, y_diff: i32, z_diff: i32) -> Cube {
        Cube::new(self.x + x_diff, self.y + y_diff, self.z + z_diff)
    }

    fn get_neighbors(&self) -> Vec<Cube> {
        let mut neighbors = vec![];
        let sides = vec![
            (0, 0, 1),
            (0, 1, 0),
            (1, 0, 0),
            (0, 0, -1),
            (0, -1, 0),
            (-1, 0, 0),
        ];
        for (x, y, z) in sides {
            neighbors.push(self.moved(x, y, z));
        }

        neighbors
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Bounds3D {
    top_left_shallow: Cube,
    bottom_right_deep: Cube,
}

impl Bounds3D {
    fn contains(&self, cube: &Cube) -> bool {
        cube.x >= self.top_left_shallow.x
            && cube.x <= self.bottom_right_deep.x
            && cube.y >= self.top_left_shallow.y
            && cube.y <= self.bottom_right_deep.y
            && cube.z >= self.top_left_shallow.z
            && cube.z <= self.bottom_right_deep.z
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Droplet {
    cubes: Vec<Cube>,
    bounds: Bounds3D,
}

impl Droplet {
    fn new(cubes: &[Cube]) -> Droplet {
        let bounds = Droplet::calculate_bounds(&cubes);
        Droplet {
            cubes: cubes.to_vec(),
            bounds,
        }
    }

    fn from_lines(lines: &[String]) -> Droplet {
        let cubes: Vec<Cube> = lines.iter().map(|l| Cube::from_str(l)).collect();

        Droplet::new(&cubes)
    }

    fn calculate_bounds(cubes: &[Cube]) -> Bounds3D {
        let xs: Vec<i32> = cubes.iter().map(|c| c.x).collect();
        let ys: Vec<i32> = cubes.iter().map(|c| c.y).collect();
        let zs: Vec<i32> = cubes.iter().map(|c| c.z).collect();

        Bounds3D {
            top_left_shallow: Cube::new(
                *xs.iter().min().unwrap(),
                *ys.iter().min().unwrap(),
                *zs.iter().min().unwrap(),
            ),
            bottom_right_deep: Cube::new(
                *xs.iter().max().unwrap(),
                *ys.iter().max().unwrap(),
                *zs.iter().max().unwrap(),
            ),
        }
    }

    fn get_bubble(&self, candidate_cube: &Cube) -> Option<Droplet> {
        let mut cubes_to_check = vec![candidate_cube.clone()];
        let mut bubble_cubes: HashSet<Cube> = cubes_to_check.iter().cloned().collect();
        let mut already_checked: HashSet<Cube> = HashSet::new();
        while !cubes_to_check.is_empty() {
            let cube = cubes_to_check.pop().unwrap();

            already_checked.insert(cube.clone());

            for neighbor in cube.get_neighbors() {
                if already_checked.contains(&neighbor) {
                    continue;
                }

                if !self.bounds.contains(&neighbor) {
                    return None;
                }

                if self.cubes.contains(&neighbor) {
                    continue;
                }

                bubble_cubes.insert(neighbor.clone());
                cubes_to_check.push(neighbor.clone());
            }
        }

        Some(Droplet::new(
            &bubble_cubes.iter().cloned().collect::<Vec<Cube>>(),
        ))
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
