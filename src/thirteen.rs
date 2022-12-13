use std::cmp::Ordering;
use std::fmt;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::iter::Peekable;
use std::str::Chars;

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = sum_indicies_in_correct_order(&values);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_decoder_key(&values);

    println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> Vec<Vec<Element>> {
    let mut entries: Vec<Vec<Element>> = Vec::new();
    let mut entry: Vec<Element> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            entries.push(entry.clone());
            entry = vec![];
            continue;
        }

        entry.push(Element::from_str(&line));
    }

    entries.push(entry.clone());

    entries
}

#[derive(Clone, PartialEq, Eq)]
enum Element {
    List(Vec<Element>),
    Value(i32),
}

impl std::fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Value(value) => f.write_str(value.to_string().as_str()),
            Element::List(list) => {
                f.write_str("[")?;
                for (i, e) in list.iter().enumerate() {
                    e.fmt(f)?;
                    if i < list.len() - 1 {
                        f.write_str(", ")?;
                    }
                }
                f.write_str("]")
            }
        }
    }
}

impl Element {
    fn from_str(s: &str) -> Element {
        let mut stack = vec![Element::List(vec![])];

        let mut char_iter = s.chars().peekable();

        loop {
            let token = Element::get_token(&mut char_iter);
            if token.is_none() {
                break;
            }

            let token = token.unwrap();
            match token.as_str() {
                "[" => stack.push(Element::List(vec![])),
                "]" => {
                    let e = stack.pop().unwrap();
                    stack.last_mut().unwrap().push(e)
                }
                "," => (),
                value => {
                    let v = Element::Value(value.parse().unwrap());
                    stack.last_mut().unwrap().push(v);
                }
            }
        }

        stack.pop().unwrap().get_list()[0].clone()
    }

    fn get_token(char_iter: &mut Peekable<Chars>) -> Option<String> {
        let digits = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

        let mut chars = vec![];

        chars.push(char_iter.next()?);
        if digits.contains(chars.last().unwrap()) {
            let next_char = char_iter.peek();

            if let Some(next_char) = next_char {
                if digits.contains(next_char) {
                    chars.push(char_iter.next().unwrap());
                }
            }
        }

        Some(chars.iter().collect())
    }

    fn push(&mut self, element: Element) {
        match self {
            Element::List(list) => list.push(element),
            _ => panic!(),
        }
    }

    fn compare(&self, other: &Element) -> Ordering {
        match (self, other) {
            (Element::Value(_), Element::Value(_)) => self.compare_values(other),
            (Element::List(_), Element::Value(_)) => self.compare_lists(&other.to_list()),
            (Element::Value(_), Element::List(_)) => self.to_list().compare_lists(other),
            (Element::List(_), Element::List(_)) => self.compare_lists(other),
        }
    }

    fn get_value(&self) -> i32 {
        match self {
            Element::Value(value) => *value,
            Element::List(_) => panic!(),
        }
    }

    fn get_list(&self) -> Vec<Element> {
        match self {
            Element::List(list) => list.clone(),
            Element::Value(_) => panic!(),
        }
    }

    fn compare_values(&self, other: &Element) -> Ordering {
        let left = self.get_value();
        let right = other.get_value();

        left.cmp(&right)
    }

    fn compare_lists(&self, other: &Element) -> Ordering {
        let s = self.get_list();
        let o = other.get_list();

        for i in 0..*vec![s.len(), o.len()].iter().max().unwrap() {
            if i >= s.len() {
                return Ordering::Less;
            }
            if i >= o.len() {
                return Ordering::Greater;
            }

            let left = s[i].clone();
            let right = o[i].clone();

            match left.compare(&right) {
                Ordering::Equal => (),
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
            }
        }

        Ordering::Equal
    }

    fn to_list(&self) -> Element {
        match self {
            Element::Value(v) => Element::List(vec![Element::Value(*v)]),
            Element::List(_) => panic!(),
        }
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn sum_indicies_in_correct_order(values: &[Vec<Element>]) -> i32 {
    values
        .iter()
        .enumerate()
        .map(|(i, lists)| (i + 1, lists[0].compare(&lists[1]) != Ordering::Greater))
        .filter(|(_, b)| *b)
        .map(|(i, _)| i)
        .sum::<usize>() as i32
}

fn find_decoder_key(values: &[Vec<Element>]) -> i32 {
    let mut packets: Vec<Element> = values.iter().flatten().cloned().collect();

    let divider_packets = vec![Element::from_str("[[2]]"), Element::from_str("[[6]]")];

    divider_packets
        .iter()
        .cloned()
        .for_each(|p| packets.push(p));

    packets.sort();

    divider_packets
        .iter()
        .map(|p| packets.iter().position(|a| a == p).unwrap() + 1)
        .product::<usize>() as i32
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    use std::fs::File;

    #[test]
    fn test_part_1_example() {
        let f = File::open("inputs/thirteen_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 13;
        let actual = sum_indicies_in_correct_order(&values);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_1_actual() {
        let f = File::open("inputs/thirteen.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 5003;
        let actual = sum_indicies_in_correct_order(&values);

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_part_2_example() {
        let f = File::open("inputs/thirteen_example.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 140;
        let actual = find_decoder_key(&values);

        assert_eq!(expected, actual)
    }

    #[ignore]
    #[test]
    fn test_part_2_actual() {
        let f = File::open("inputs/thirteen.txt").unwrap();
        let values = read_input(&mut BufReader::new(f));

        let expected = 20280;
        let actual = find_decoder_key(&values);

        assert_eq!(expected, actual)
    }
}
