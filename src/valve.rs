mod utils;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::vec;

use utils::utils::read_file;
use utils::utils::Node;

const DEPTH_FILE_NAME: &str = "inputs/valve.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    let graph: HashMap<String, Node> = input.lines().map(|l| parse_line(l)).collect();
    let mut goals: Vec<Node> = graph
        .iter()
        .filter(|(_, v)| v.flow > 0)
        .map(|(_, v)| v.clone())
        .collect::<Vec<Node>>();
    // println!("{}", multiple_searchs(&graph, &mut goals));
    let res = calculate_best_path(&graph, &goals);

    // let mut visited = VecDeque::new();
    // visited.push_back(graph.get("AA").unwrap().clone());
    // let res = dfs(&graph, &"AA".to_string(), visited, 0, 0, &"DD".to_string());
    println!("{:?}", res);
    println!("{}", calculate_best_path_with_elephant(&graph, &goals));
}

fn ratio(path: &Vec<(Node, u32, u32)>) -> f32 {
    let flow = path.first().unwrap().1; //path.iter().fold(0, |accum, t| t.1 + accum);
    let time = path.first().unwrap().2;
    flow as f32 / time as f32
}

fn calculate_distances(
    graph: &HashMap<String, Node>,
    goals: &Vec<Node>,
) -> HashMap<String, Vec<Vec<(Node, u32, u32)>>> {
    let mut paths: HashMap<String, Vec<Vec<(Node, u32, u32)>>> = HashMap::new();
    for goal in goals {
        for (k, _) in graph {
            if goal.id == *k {
                continue;
            }
            let mut visited = HashSet::new();
            visited.insert(graph.get(k).unwrap().clone());
            let mut path = dfs(graph, k, visited, 0, 0, &goal.id);
            path.pop();
            match paths.get_mut(k) {
                Some(val) => val.push(path),
                None => {
                    let mut vec = Vec::new();
                    vec.push(path);
                    paths.insert(k.clone(), vec);
                }
            };
        }
        println!("{:?}", goal);
    }
    paths
}

fn find_best_path(
    paths: &HashMap<String, Vec<Vec<(Node, u32, u32)>>>,
    goals: &Vec<Node>,
    start: &String,
    minutes_left: i32,
) -> i32 {
    let mut possible_solutions: Vec<i32> = Vec::new();
    for goal in goals {
        let path_to_goal: Vec<(Node, u32, u32)> = paths[start]
            .clone()
            .into_iter()
            .filter(|p| p[0].0.id == *goal.id)
            .collect::<Vec<Vec<_>>>()[0]
            .clone();
        let destination = path_to_goal[0].clone();
        let distance = destination.2;
        if distance as i32 >= minutes_left {
            continue;
        }
        let minutes_left: i32 = minutes_left - distance as i32 - 1;
        let flow = destination.0.flow as i32 * minutes_left;
        let next_goals: Vec<Node> = goals
            .clone()
            .into_iter()
            .filter(|g| g.id != destination.0.id)
            .collect();
        println!("{:?}", next_goals);
        let accum = flow + find_best_path(paths, &next_goals, &destination.0.id, minutes_left);
        possible_solutions.push(accum);
    }
    *possible_solutions.iter().max().unwrap_or(&0)
}

fn find_best_path_with_elephant(
    paths: &HashMap<String, Vec<Vec<(Node, u32, u32)>>>,
    goals: &Vec<Node>,
    start: &String,
    minutes_left: i32,
) -> Vec<i32> {
    let mut possible_solutions: Vec<i32> = Vec::new();
    for goal in goals {
        let path_to_goal: Vec<(Node, u32, u32)> = paths[start]
            .clone()
            .into_iter()
            .filter(|p| p[0].0.id == *goal.id)
            .collect::<Vec<Vec<_>>>()[0]
            .clone();
        let destination = path_to_goal[0].clone();
        let distance = destination.2;
        if distance as i32 >= minutes_left {
            continue;
        }
        let minutes_left: i32 = minutes_left - distance as i32 - 1;
        let flow = destination.0.flow as i32 * minutes_left;
        let next_goals: Vec<Node> = goals
            .clone()
            .into_iter()
            .filter(|g| g.id != destination.0.id)
            .collect();
        let accum = flow + find_best_path(paths, &next_goals, &destination.0.id, minutes_left);
        possible_solutions.push(accum);
    }
    possible_solutions.sort();
    possible_solutions.reverse();
    possible_solutions
}

fn calculate_best_path_with_elephant(graph: &HashMap<String, Node>, goals: &Vec<Node>) -> i32 {
    let goals = goals.clone();
    let paths = calculate_distances(graph, &goals);
    let start = "AA".to_string();
    let ps = find_best_path_with_elephant(&paths, &goals, &start, 26);
    println!("{}", ps[0] + ps[1]);
    0i32
}

fn calculate_best_path(graph: &HashMap<String, Node>, goals: &Vec<Node>) -> i32 {
    let goals = goals.clone();
    let paths = calculate_distances(graph, &goals);
    let start = "AA".to_string();
    find_best_path(&paths, &goals, &start, 30)
}

fn dfs(
    graph: &HashMap<String, Node>,
    position: &String,
    visited: HashSet<Node>,
    accum: u32,
    time: u32,
    goal: &String,
) -> Vec<(Node, u32, u32)> {
    let mut visited = visited.clone();
    let node = graph.get(position).unwrap();
    let mut best_path = Vec::new();
    let default_node = (get_default(), 0, u32::MAX);
    if time < 30 {
        for n in node.neighbors.iter() {
            let neighbor_node = graph.get(n).unwrap();
            if visited.insert(neighbor_node.clone()) {
                let neighbor = dfs(graph, n, visited.clone(), 0, time + 1, goal);
                if best_path.first().unwrap_or(&default_node).2 > neighbor.first().unwrap().2
                    && neighbor.first().unwrap().0.id == *goal
                {
                    best_path = neighbor;
                }
                visited.remove(neighbor_node);
            }
        }
    }

    best_path.push((node.clone(), 0, time));

    best_path
}

fn parse_line(line: &str) -> (String, Node) {
    let line = line.replace("valves", "valve");
    let mut split = line.split(";");
    let id = line
        .split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .to_string();
    let flow = split
        .next()
        .unwrap()
        .split("=")
        .skip(1)
        .next()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let neighbors = split
        .next()
        .unwrap()
        .split("valve")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.trim())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    (
        id.clone(),
        Node {
            id: id.clone(),
            flow,
            neighbors,
        },
    )
}

fn get_default() -> Node {
    Node {
        id: "".to_string(),
        flow: 0,
        neighbors: Vec::new(),
    }
}
