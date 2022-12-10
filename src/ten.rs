use std::collections::HashSet;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = calc_sum_signal_strengths(&values);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = build_image(&values);

    println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in reader.lines() {
        instructions.push(Instruction::from_str(&line.unwrap()));
    }

    instructions
}

struct Registers {
    x: i32,
}

impl Registers {
    fn new() -> Registers {
        Registers { x: 1 }
    }

    fn execute(&mut self, inst: &Instruction) {
        match inst {
            Instruction::Noop => (),
            Instruction::Addx(a) => self.x += a,
        }
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        if s == "noop" {
            Instruction::Noop
        } else {
            Instruction::Addx(s.split(' ').last().unwrap().parse().unwrap())
        }
    }

    fn duration(&self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

fn calc_sum_signal_strengths(instructions: &[Instruction]) -> i32 {
    let mut registers = Registers::new();

    let important_cycles: HashSet<i32> = vec![20, 60, 100, 140, 180, 220].iter().cloned().collect();

    let mut signals_sum = 0;

    let mut instruction_set: Vec<(Instruction, u32)> = instructions
        .iter()
        .map(|i| (i.clone(), i.duration()))
        .collect();
    let mut cycle = 1;
    while !instruction_set.is_empty() {
        instruction_set.first_mut().unwrap().1 -= 1;

        if important_cycles.contains(&cycle) {
            let signal_strength = cycle * registers.x;
            signals_sum += signal_strength;
        }

        if instruction_set.first().unwrap().1 == 0 {
            let instruction = instruction_set.first().unwrap().0.clone();
            instruction_set.remove(0);

            registers.execute(&instruction);
        }

        cycle += 1;
    }

    signals_sum
}

fn build_image(instructions: &[Instruction]) -> String {
    let mut registers = Registers::new();

    let mut image_chars: Vec<char> = vec![];

    let mut instruction_set: Vec<(Instruction, u32)> = instructions
        .iter()
        .map(|i| (i.clone(), i.duration()))
        .collect();
    let mut cycle = 1;
    while !instruction_set.is_empty() {
        instruction_set.first_mut().unwrap().1 -= 1;

        let column = (cycle - 1) % 40;
        if (registers.x - 1..registers.x + 2).contains(&column) {
            image_chars.push('#');
        } else {
            image_chars.push('.');
        }

        if column == 39 {
            image_chars.push('\n');
        }

        if instruction_set.first().unwrap().1 == 0 {
            let instruction = instruction_set.first().unwrap().0.clone();
            instruction_set.remove(0);

            registers.execute(&instruction);
        }

        cycle += 1;
    }

    image_chars.iter().collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::fs::File;

    #[test]
    fn test_part_1_example() {
        let f = File::open("inputs/ten_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 13140;
        let actual = calc_sum_signal_strengths(&values);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_1_actual() {
        let f = File::open("inputs/ten.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 14920;
        let actual = calc_sum_signal_strengths(&values);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_2_example() {
        let f = File::open("inputs/ten_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = concat!(
            "##..##..##..##..##..##..##..##..##..##..\n",
            "###...###...###...###...###...###...###.\n",
            "####....####....####....####....####....\n",
            "#####.....#####.....#####.....#####.....\n",
            "######......######......######......####\n",
            "#######.......#######.......#######.....\n",
        );
        let actual = build_image(&values);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_2_actual() {
        let f = File::open("inputs/ten.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = concat!(
            "###..#..#..##...##...##..###..#..#.####.\n",
            "#..#.#..#.#..#.#..#.#..#.#..#.#..#....#.\n",
            "###..#..#.#....#..#.#....###..#..#...#..\n",
            "#..#.#..#.#....####.#....#..#.#..#..#...\n",
            "#..#.#..#.#..#.#..#.#..#.#..#.#..#.#....\n",
            "###...##...##..#..#..##..###...##..####.\n",
        );
        let actual = build_image(&values);

        assert_eq!(expected, actual)
    }
}
