use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = sum_all_quality_levels(&values);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = multiply_max_geodes(&values);

    println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> Vec<Blueprint> {
    let mut blueprints: Vec<Blueprint> = Vec::new();
    for line in reader.lines() {
        blueprints.push(Blueprint::from_str(&line.unwrap()));
    }

    blueprints
}

fn sum_all_quality_levels(blueprints: &[Blueprint]) -> i32 {
    let num_steps = 24;
    let start_state = State::new(
        vec![(Resource::OreRobot, 1)].into_iter().collect(),
        num_steps,
    );

    blueprints
        .iter()
        .enumerate()
        .map(|(i, b)| calc_largest_geode_total(b, &start_state) * (i as i32 + 1))
        .sum()
}

fn calc_largest_geode_total(blueprint: &Blueprint, starting_state: &State) -> i32 {
    let mut states_to_visit: BinaryHeap<(State, Path)> =
        vec![(starting_state.clone(), Path::new())]
            .into_iter()
            .collect();
    let mut visited_states: HashSet<State> = HashSet::new();

    let mut max_geodes = 0;

    while !states_to_visit.is_empty() {
        let (state, path) = states_to_visit.pop().unwrap();

        if visited_states.contains(&state) {
            continue;
        }
        visited_states.insert(state.clone());

        let num_geodes = *state.resources.get(&Resource::Geode).unwrap_or(&0);
        if num_geodes > max_geodes {
            max_geodes = num_geodes;
        }

        let best_case_num_geodes = state.get_best_case_num_geodes();
        if best_case_num_geodes <= max_geodes {
            continue;
        }

        if state.steps_remaining >= 1 {
            for action in state.get_possible_actions(blueprint) {
                let new_state = state.step(action, blueprint);

                if !visited_states.contains(&new_state) {
                    states_to_visit.push((new_state, path.appended(&action)));
                }
            }
        }
    }

    max_geodes
}

fn multiply_max_geodes(blueprints: &[Blueprint]) -> i32 {
    let blueprints: Vec<Blueprint> = blueprints.iter().take(3).cloned().collect();

    let num_steps = 32;
    let start_state = State::new(
        vec![(Resource::OreRobot, 1)].into_iter().collect(),
        num_steps,
    );

    blueprints
        .iter()
        .map(|b| calc_largest_geode_total(b, &start_state))
        .product()
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Path {
    valves: Vec<Action>,
}

impl Path {
    fn new() -> Path {
        Path { valves: vec![] }
    }

    fn appended(&self, node: &Action) -> Path {
        let mut valves = self.valves.clone();
        valves.push(*node);

        Path { valves }
    }
}

#[derive(Debug, Clone)]
struct Blueprint {
    costs: HashMap<Resource, HashSet<(Resource, i32)>>,
}

impl Blueprint {
    fn from_str(s: &str) -> Blueprint {
        let parts: Vec<&str> = s.split('.').collect();

        let mut costs = HashMap::new();

        let ore_robot_cost: i32 = parts[0]
            .split("costs ")
            .last()
            .unwrap()
            .split(" ore")
            .next()
            .unwrap()
            .parse()
            .unwrap();
        costs.insert(
            Resource::OreRobot,
            vec![(Resource::Ore, ore_robot_cost)].into_iter().collect(),
        );

        let clay_robot_cost: i32 = parts[1]
            .split("costs ")
            .last()
            .unwrap()
            .split(" ore")
            .next()
            .unwrap()
            .parse()
            .unwrap();
        costs.insert(
            Resource::ClayRobot,
            vec![(Resource::Ore, clay_robot_cost)].into_iter().collect(),
        );

        let obsidian_robot_cost_1: i32 = parts[2]
            .split("costs ")
            .last()
            .unwrap()
            .split(" ore")
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let obsidian_robot_cost_2: i32 = parts[2]
            .split("costs ")
            .last()
            .unwrap()
            .split(" and ")
            .last()
            .unwrap()
            .split(" clay")
            .next()
            .unwrap()
            .parse()
            .unwrap();
        costs.insert(
            Resource::ObsidianRobot,
            vec![
                (Resource::Ore, obsidian_robot_cost_1),
                (Resource::Clay, obsidian_robot_cost_2),
            ]
            .into_iter()
            .collect(),
        );

        let geode_robot_cost_1: i32 = parts[3]
            .split("costs ")
            .last()
            .unwrap()
            .split(" ore")
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let geode_robot_cost_2: i32 = parts[3]
            .split("costs ")
            .last()
            .unwrap()
            .split(" and ")
            .last()
            .unwrap()
            .split(" obsidian")
            .next()
            .unwrap()
            .parse()
            .unwrap();
        costs.insert(
            Resource::GeodeRobot,
            vec![
                (Resource::Ore, geode_robot_cost_1),
                (Resource::Obsidian, geode_robot_cost_2),
            ]
            .into_iter()
            .collect(),
        );

        Blueprint { costs }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Resource {
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodeRobot,
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    fn get_mined_resource(&self) -> Option<Resource> {
        match self {
            Resource::OreRobot => Some(Resource::Ore),
            Resource::ClayRobot => Some(Resource::Clay),
            Resource::ObsidianRobot => Some(Resource::Obsidian),
            Resource::GeodeRobot => Some(Resource::Geode),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    resources: BTreeMap<Resource, i32>,
    steps_remaining: i32,
}

impl State {
    fn new(resources: BTreeMap<Resource, i32>, steps_remaining: i32) -> State {
        State {
            resources,
            steps_remaining,
        }
    }

    fn step(&self, action: Action, blueprint: &Blueprint) -> State {
        let mut pre_step_resources = self.resources.clone();

        if let Action::Build(resource) = action {
            for (component, count) in blueprint.costs.get(&resource).unwrap().iter() {
                *pre_step_resources.get_mut(component).unwrap() -= count;
            }
        }

        let mut new_resources = pre_step_resources.clone();
        let new_steps_remaining = self.steps_remaining - 1;

        for (resource, count) in pre_step_resources.iter() {
            if let Some(mined_resource) = resource.get_mined_resource() {
                *new_resources.entry(mined_resource).or_insert(0) += count;
            }
        }

        if let Action::Build(resource) = action {
            *new_resources.entry(resource).or_insert(0) += 1;
        }

        State::new(new_resources, new_steps_remaining)
    }

    fn get_possible_actions(&self, blueprint: &Blueprint) -> Vec<Action> {
        let mut actions = vec![Action::DontBuild];

        for resource in blueprint.costs.keys() {
            if self.can_build(blueprint, *resource) {
                actions.push(Action::Build(*resource));
            }
        }

        actions.reverse();

        actions
    }

    fn can_build(&self, blueprint: &Blueprint, resource: Resource) -> bool {
        let needed_components = blueprint.costs.get(&resource).unwrap();

        for (component, amount) in needed_components.iter() {
            if self.resources.get(component).unwrap_or(&0) < amount {
                return false;
            }
        }

        true
    }

    fn get_best_case_num_geodes(&self) -> i32 {
        let mut num_geodes = *self.resources.get(&Resource::Geode).unwrap_or(&0);
        let mut num_geode_robots = *self.resources.get(&Resource::GeodeRobot).unwrap_or(&0);
        for _ in 0..self.steps_remaining {
            num_geodes += num_geode_robots;
            num_geode_robots += 1;
        }

        num_geodes
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.resources.get(&Resource::Geode).unwrap_or(&0),
            &self.resources,
            self.steps_remaining,
        )
            .cmp(&(
                other.resources.get(&Resource::Geode).unwrap_or(&0),
                &other.resources,
                other.steps_remaining,
            ))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Action {
    Build(Resource),
    DontBuild,
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
