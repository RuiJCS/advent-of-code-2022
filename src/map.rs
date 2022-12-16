mod utils;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

use utils::utils::process_lines;
use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/map.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    let map =
        input
            .lines()
            .enumerate()
            .fold((Vec::<Vec<u8>>::new(), (0, 0), (0, 0)), |mut acc, s| {
                acc.0.push(
                    s.1.chars()
                        .enumerate()
                        .fold(Vec::<u8>::new(), |mut acc1, c| {
                            if c.1 == 'S' {
                                acc.1 = (c.0, s.0);
                                acc1.push('a' as u8 - 97);
                            } else if c.1 == 'E' {
                                acc.2 = (c.0, s.0);
                                acc1.push('z' as u8 - 97);
                            } else {
                                acc1.push(c.1.to_ascii_lowercase() as u8 - 97);
                            }
                            acc1
                        }),
                );
                acc
            });

    let result1 = breadth_first_search(&map.0, map.1, map.2);
    println!("{}", result1);
    println!("{}", multiple_searchs(&map.0, map.2))
}

fn neighbors(
    position: &(usize, usize),
    max_depth: usize,
    max_height: usize,
) -> Vec<(usize, usize, char)> {
    let mut res = Vec::new();
    if position.0 < max_depth {
        res.push((position.0 + 1, position.1, 'V'));
    }
    if position.0 > 0 {
        res.push((position.0 - 1, position.1, '^'));
    }
    if position.1 < max_height {
        res.push((position.0, position.1 + 1, '>'));
    }
    if position.1 > 0 {
        res.push((position.0, position.1 - 1, '<'));
    }
    res
}

fn breadth_first_search(graph: &Vec<Vec<u8>>, root: (usize, usize), goal: (usize, usize)) -> u32 {
    let max_height = graph.len() - 1;
    let max_depth = graph[0].len() - 1;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue = VecDeque::<((usize, usize), u32)>::new();
    visited.insert(root);

    queue.push_back((root, 0));
    while let Some(position) = queue.pop_front() {
        if position.0 == goal {
            return position.1;
        }

        for neighbor in neighbors(&position.0, max_depth, max_height) {
            if !visited.contains(&(neighbor.0, neighbor.1))
                && graph[neighbor.1][neighbor.0] as i32 - graph[position.0 .1][position.0 .0] as i32
                    <= 1
            {
                visited.insert((neighbor.0, neighbor.1));
                queue.push_back(((neighbor.0, neighbor.1), position.1 + 1));
            }
        }
    }

    u32::MAX
}

fn multiple_searchs(graph: &Vec<Vec<u8>>, goal: (usize, usize)) -> u32 {
    let mut min = u32::MAX;
    for v in graph.iter().enumerate() {
        for n in v.1.iter().enumerate() {
            if *n.1 == 0 {
                let distance = breadth_first_search(graph, (n.0, v.0), goal);
                if distance < min {
                    min = distance;
                }
            }
        }
    }
    min
}
