use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_total_size(&values);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_smallest_to_remove(&values);

    println!("{}", answer);
}

#[derive(Debug)]
enum Command {
    CdUp(),
    CdRoot(),
    Cd(String),
    Ls(Vec<Object>),
}

impl Command {
    fn from_lines(lines: &[String]) -> Command {
        match lines.first().unwrap().as_str() {
            "$ cd /" => return Command::CdRoot(),
            "$ cd .." => return Command::CdUp(),
            _ => (),
        }

        if lines.first().unwrap().starts_with("$ cd ") {
            let line = lines.first().unwrap();

            return Command::Cd(line.split("cd ").last().unwrap().to_string());
        }

        Command::Ls(lines.iter().skip(1).map(|l| Object::from_line(l)).collect())
    }
}

#[derive(Debug)]
enum Object {
    File(String, i32),
    Directory(String),
}

impl Object {
    fn from_line(line: &str) -> Object {
        if line.starts_with("dir") {
            Object::Directory(
                line.split(' ')
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .to_string(),
            )
        } else {
            let parts = line.split(' ').collect::<Vec<&str>>();

            let name = parts.last().unwrap().to_string();
            let size = parts.first().unwrap().parse().unwrap();

            Object::File(name, size)
        }
    }
}

#[derive(Debug)]
enum Structure {
    File(String, i32),
    Directory(String, Vec<Structure>),
}

impl Structure {
    fn get_size(&self) -> i32 {
        match self {
            Structure::File(_, size) => *size,
            Structure::Directory(_, members) => members.iter().map(Structure::get_size).sum(),
        }
    }

    fn from_commands(commands: &[Command]) -> Structure {
        let mut root = Structure::Directory("/".to_string(), Vec::new());

        let mut current_dir = &mut root;
        let mut path: Vec<String> = Vec::new();
        for comm in commands.iter() {
            match comm {
                Command::CdRoot() => current_dir = &mut root,
                Command::CdUp() => {
                    path.pop();
                    current_dir = root.get_path(&path);
                }
                Command::Cd(name) => {
                    path.push(name.to_string());
                    current_dir = current_dir.get_child_mut(name);
                }
                Command::Ls(objects) => {
                    objects
                        .iter()
                        .map(|o| current_dir.add_child(o))
                        .for_each(drop);
                }
            }
        }

        root
    }

    fn add_child(&mut self, child: &Object) {
        match self {
            Structure::Directory(_, children) => match child {
                Object::File(name, size) => children.push(Structure::File(name.to_string(), *size)),
                Object::Directory(name) => {
                    children.push(Structure::Directory(name.to_string(), Vec::new()))
                }
            },
            Structure::File(_, _) => panic!(),
        }
    }

    fn get_path(&mut self, path: &[String]) -> &mut Structure {
        path.iter().fold(self, |s, n| s.get_child_mut(n))
    }

    fn get_child_mut(&mut self, name: &str) -> &mut Structure {
        match self {
            Structure::Directory(_, children) => children
                .iter_mut()
                .find(|c| match c {
                    Structure::Directory(name_2, _) => name_2 == name,
                    _ => false,
                })
                .unwrap(),
            _ => panic!(),
        }
    }

    fn get_child(&self, name: &str) -> &Structure {
        match self {
            Structure::Directory(_, children) => children
                .iter()
                .find(|c| match c {
                    Structure::Directory(name_2, _) => name_2 == name,
                    _ => false,
                })
                .unwrap(),
            _ => panic!(),
        }
    }

    fn get_subdirs(&self) -> Vec<String> {
        match self {
            Structure::Directory(_, children) => {
                let mut subdirs = Vec::new();
                for child in children {
                    if let Structure::Directory(name, _) = child {
                        subdirs.push(name.to_string());
                    }
                }

                subdirs
            }
            Structure::File(_, _) => panic!(),
        }
    }
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();
    let mut buffer: Vec<String> = Vec::new();

    let mut seen_any = false;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.starts_with('$') && seen_any {
            commands.push(Command::from_lines(&buffer));
            buffer.clear();
        }

        buffer.push(line.to_string());
        seen_any = true;
    }

    commands.push(Command::from_lines(&buffer));

    commands
}

fn find_total_size(commands: &[Command]) -> i32 {
    let structure = Structure::from_commands(commands);

    let mut dirs_to_check = Vec::new();
    dirs_to_check.push(&structure);

    let mut total = 0;
    while !dirs_to_check.is_empty() {
        let dir = dirs_to_check.pop().unwrap();
        let size = dir.get_size();

        if size < 100000 {
            total += size;
        }

        for d in dir.get_subdirs() {
            let subdir = dir.get_child(&d);
            dirs_to_check.push(subdir);
        }
    }

    total
}

fn find_smallest_to_remove(commands: &[Command]) -> i32 {
    let structure = Structure::from_commands(commands);

    let mut dirs_to_check = Vec::new();
    dirs_to_check.push(&structure);

    let mut smallest_to_remove = 999999999;
    while !dirs_to_check.is_empty() {
        let dir = dirs_to_check.pop().unwrap();
        let size = dir.get_size();

        if size >= (structure.get_size() - 40000000) && size < smallest_to_remove {
            smallest_to_remove = size;
        }

        for d in dir.get_subdirs() {
            let subdir = dir.get_child(&d);
            dirs_to_check.push(subdir);
        }
    }

    smallest_to_remove
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::fs::File;

    #[test]
    fn test_part_1_example() {
        let f = File::open("inputs/seven_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 95437;
        let actual = find_total_size(&values);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_1_actual() {
        let f = File::open("inputs/seven.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 1447046;
        let actual = find_total_size(&values);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_2_example() {
        let f = File::open("inputs/seven_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 24933642;
        let actual = find_smallest_to_remove(&values);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_2_actual() {
        let f = File::open("inputs/seven.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 578710;
        let actual = find_smallest_to_remove(&values);

        assert_eq!(expected, actual)
    }
}
