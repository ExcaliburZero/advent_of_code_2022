use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn part_one() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    let answer = find_max_pressure(&values);

    println!("{}", answer);
}

pub fn part_two() {
    let values = read_input(&mut BufReader::new(io::stdin()));
    //let answer = find_start_marker_2(&values[0]);

    //println!("{}", answer);
}

fn read_input<T: std::io::Read>(reader: &mut BufReader<T>) -> HashMap<String, Valve> {
    let mut valves: HashMap<String, Valve> = HashMap::new();
    for line in reader.lines() {
        let valve = Valve::from_str(&line.unwrap());
        //println!("{valve:?}");
        valves.insert(valve.name.clone(), valve);
    }

    valves
}

fn find_max_pressure(valves: &HashMap<String, Valve>) -> i32 {
    /*
    let starting_valve = valves.get("AA").unwrap();
    let total_time = 30;

    let starting_state = State::new(&starting_valve.name, total_time, 0, BTreeMap::new());

    let mut state = starting_state.apply_action(&Action::Move("DD".to_string()), valves);
    //println!("{state:?}");
    state = state.apply_action(&Action::Open, valves);
    //println!("{state:?}");

    //panic!();

    let mut states_to_try: BTreeSet<State> = BTreeSet::new();
    states_to_try.insert(starting_state);

    let mut tried_states: BTreeSet<State> = BTreeSet::new();

    let mut max_score = 0;
    while !states_to_try.is_empty() {
        let state = states_to_try.iter().next().unwrap().clone();
        states_to_try.remove(&state);

        if tried_states.contains(&state) {
            continue;
        }
        tried_states.insert(state.clone());

        if state.best_case_score(valves) < max_score {
            continue;
        }

        if state.score > max_score {
            println!("{state:?}");
            max_score = state.score;
        }

        if state.time_remaining == 0 {
            continue;
        }

        for action in state.get_possible_actions(valves) {
            let new_state = state.apply_action(&action, valves);
            if !tried_states.contains(&new_state) {
                states_to_try.insert(new_state);
            }
        }
    }

    max_score*/

    let meta_graph = MetaGraph::from_valves(valves);
    println!("{meta_graph:?}");

    let start = "AA".to_string();
    let all_nodes: Vec<String> = valves.keys().cloned().collect();

    let starting_state = SearchState::new();

    let mut states_to_try: BTreeSet<SearchState> = BTreeSet::new();
    states_to_try.insert(starting_state);

    let mut tried_states: BTreeSet<SearchState> = BTreeSet::new();

    let mut max_score = 0;
    while !states_to_try.is_empty() {
        let state = states_to_try.iter().next().unwrap().clone();
        states_to_try.remove(&state);

        //println!("trying: {state:?}");

        if tried_states.contains(&state) {
            //println!("already_visited");
            continue;
        }
        tried_states.insert(state.clone());

        let (state_result, actions) = state.to_game_state(&start, 30, &meta_graph, valves);
        //println!("\t{state_result:?}");

        if state_result.time_remaining < 0 {
            //println!("too long");
            continue;
        }

        if state_result.best_case_score(valves) < max_score {
            //println!("best score is too low");
            continue;
        }

        if state_result.score > max_score {
            println!("-------------------");
            println!("{state:?}");
            println!("{actions:?}");
            println!("{state_result:?}");
            max_score = state_result.score;
        }

        for node in state.get_remaining_nodes(&all_nodes) {
            let new_state = state.appended(&node);
            if !tried_states.contains(&new_state) {
                states_to_try.insert(new_state);
            }
        }
    }

    for (entry, value) in meta_graph.edges.iter() {
        println!("{entry:?} => {value:?}");
    }

    max_score
}

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Clone)]
struct State {
    location: String,
    time_remaining: i32,
    score: i32,
    valve_states: BTreeMap<String, bool>,
}

/*
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.location, &self.time_remaining).cmp(&(other.value, &other.name))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}*/

impl State {
    fn new(
        location: &String,
        time_remaining: i32,
        score: i32,
        valve_states: BTreeMap<String, bool>,
    ) -> State {
        State {
            location: location.clone(),
            time_remaining,
            score,
            valve_states,
        }
    }

    fn get_possible_actions(&self, valves: &HashMap<String, Valve>) -> Vec<Action> {
        let mut actions: Vec<Action> = valves
            .get(&self.location)
            .unwrap()
            .leads_to
            .iter()
            .map(|v| Action::Move(v.clone()))
            .collect();

        if !self.valve_states.get(&self.location).unwrap_or(&false) {
            actions.push(Action::Open);
        }

        actions
    }

    fn apply_action(&self, action: &Action, valves: &HashMap<String, Valve>) -> State {
        let new_time_remaining = self.time_remaining - 1;

        match action {
            Action::Move(valve) => State::new(
                valve,
                new_time_remaining,
                self.score,
                self.valve_states.clone(),
            ),
            Action::Open => {
                let mut new_valve_states = self.valve_states.clone();
                new_valve_states.insert(self.location.clone(), true);

                State::new(
                    &self.location,
                    new_time_remaining,
                    self.score + new_time_remaining * valves.get(&self.location).unwrap().flow_rate,
                    new_valve_states,
                )
            }
        }
    }

    fn best_case_score(&self, valves: &HashMap<String, Valve>) -> i32 {
        let mut off_valves: Vec<&Valve> = valves
            .values()
            .filter(|v| !*self.valve_states.get(&v.name).unwrap_or(&false))
            .collect();

        off_valves.sort_by(|a, b| a.flow_rate.cmp(&b.flow_rate));
        off_valves.reverse();

        let mut best_score = self.score;
        for (i, valve) in off_valves.iter().enumerate() {
            //let time = self.time_remaining - (i as i32) * 1;
            let time = self.time_remaining - (i as i32) * 2;

            if time > 0 {
                best_score += time * valve.flow_rate;
            }
        }

        best_score

        //self.score + valves.keys().filter(|v| !*self.valve_states.get(*v).unwrap_or(&false)).map(|v| valves.get(v).unwrap().flow_rate).sum::<i32>() * self.time_remaining
    }
}

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Clone)]
struct SearchState {
    visited_nodes: Vec<String>,
}

impl SearchState {
    fn new() -> SearchState {
        SearchState {
            visited_nodes: vec![],
        }
    }

    fn appended(&self, node: &String) -> SearchState {
        let mut visited_nodes = self.visited_nodes.clone();
        visited_nodes.push(node.clone());

        SearchState { visited_nodes }
    }

    fn get_remaining_nodes(&self, all_nodes: &[String]) -> Vec<String> {
        all_nodes
            .iter()
            .filter(|n| !self.visited_nodes.contains(n))
            .cloned()
            .collect()
    }

    fn to_game_state(
        &self,
        start: &String,
        start_time: i32,
        meta_graph: &MetaGraph,
        valves: &HashMap<String, Valve>,
    ) -> (State, Vec<Action>) {
        let mut state = State::new(&start, start_time, 0, BTreeMap::new());
        let mut actions = vec![];

        let mut prev_node = start;
        for node in self.visited_nodes.iter() {
            let path: &Path = meta_graph
                .edges
                .get(&(prev_node.clone(), node.clone()))
                .unwrap()
                .as_ref()
                .unwrap();

            let mut new_actions = path.to_actions();

            state = new_actions
                .iter()
                .fold(state, |s, a| s.apply_action(a, valves));

            actions.append(&mut new_actions);

            prev_node = path.valves.last().unwrap();
        }

        (state, actions)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Path {
    valves: Vec<String>,
}

impl Path {
    fn new() -> Path {
        Path { valves: vec![] }
    }

    fn appended(&self, node: &String) -> Path {
        let mut valves = self.valves.clone();
        valves.push(node.clone());

        Path { valves }
    }

    fn len(&self) -> usize {
        self.valves.len()
    }

    fn to_actions(&self) -> Vec<Action> {
        let mut actions: Vec<Action> = self
            .valves
            .iter()
            .map(|v| Action::Move(v.clone()))
            .collect();
        actions.push(Action::Open);

        actions
    }
}

#[derive(Debug)]
struct MetaGraph {
    edges: BTreeMap<(String, String), Option<Path>>,
}

impl MetaGraph {
    fn from_valves(valves: &HashMap<String, Valve>) -> MetaGraph {
        let mut edges = BTreeMap::new();
        for v1 in valves.values() {
            for v2 in valves.values() {
                edges.insert(
                    (v1.name.clone(), v2.name.clone()),
                    MetaGraph::bfs(&v1.name, &v2.name, valves),
                );
            }
        }

        MetaGraph { edges }
    }

    fn bfs(source: &String, destination: &String, valves: &HashMap<String, Valve>) -> Option<Path> {
        let mut visited: HashSet<String> = HashSet::new();
        // \/ \/ \/ Comment out to allow self revisit
        //visited.insert(source.clone());

        let mut to_visit: Vec<(String, Path)> = vec![];
        for neighbor in valves.get(source).unwrap().leads_to.iter() {
            to_visit.push((neighbor.clone(), Path::new().appended(neighbor)));
        }

        while !to_visit.is_empty() {
            let (current, path) = to_visit.remove(0);

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            if current == *destination {
                return Some(path);
            }

            for neighbor in valves.get(&current).unwrap().leads_to.iter() {
                to_visit.push((neighbor.clone(), path.appended(neighbor)));
            }
        }

        None
    }
}

#[derive(Debug)]
enum Action {
    Open,
    Move(String),
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: i32,
    leads_to: Vec<String>,
}

impl Valve {
    fn from_str(s: &str) -> Valve {
        let parts: Vec<&str> = s.split("; ").collect();

        let name = parts[0]
            .split(" has")
            .next()
            .unwrap()
            .split("Valve ")
            .last()
            .unwrap()
            .to_string();
        let flow_rate: i32 = parts[0].split("rate=").last().unwrap().parse().unwrap();

        let leads_to: Vec<String> = parts[1]
            .split(", ")
            .enumerate()
            .map(|(i, v)| {
                if i == 0 {
                    v.split(' ').last().unwrap().to_string()
                } else {
                    v.to_string()
                }
            })
            .collect();

        Valve {
            name,
            flow_rate,
            leads_to,
        }
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
