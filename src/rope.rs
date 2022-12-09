mod utils;
use std::collections::HashMap;
use std::collections::HashSet;
use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/rope.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    println!("{}", moves_n(&input, 1));
    println!("{}", moves_n(&input, 9));
}

fn moves_n(input: &String, n: usize) -> usize {
    let mut knots = HashMap::<usize, (i32, i32)>::with_capacity(n + 1);
    for i in 0..n + 1 {
        knots.insert(i, (0i32, 0i32));
    }
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let direction = split.next();
        let steps = split.next().unwrap().parse::<u32>().unwrap();
        match direction {
            Some(d) => match d {
                "U" => visited_positions.extend(process_moves(&mut knots, n, steps, 1, 0)),
                "D" => visited_positions.extend(process_moves(&mut knots, n, steps, -1, 0)),
                "R" => visited_positions.extend(process_moves(&mut knots, n, steps, 0, 1)),
                "L" => visited_positions.extend(process_moves(&mut knots, n, steps, 0, -1)),
                _ => {}
            },
            None => {}
        }
    }
    visited_positions.len() + 1
}

fn process_moves(
    knots: &mut HashMap<usize, (i32, i32)>,
    n: usize,
    steps: u32,
    step_x: i32,
    step_y: i32,
) -> HashSet<(i32, i32)> {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    for _ in 0..steps {
        let mut head = *knots.get_mut(&0).unwrap();
        let mut tail = *knots.get_mut(&1).unwrap();
        moving_n(step_x, step_y, &mut head, &mut tail);
        knots.insert(0, head);
        for i in 1..n + 1 {
            let mut head = *knots.get(&(i - 1)).unwrap();
            let mut tail = *knots.get(&i).unwrap();
            let res = moving_n(0, 0, &mut head, &mut tail);
            knots.insert(i, tail);
            if i == n {
                visited_positions.extend(res);
            }
        }
    }
    visited_positions
}

fn moving_n(
    steps_x: i32,
    steps_y: i32,
    head: &mut (i32, i32),
    tail: &mut (i32, i32),
) -> HashSet<(i32, i32)> {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    head.0 += steps_x;
    head.1 += steps_y;
    let distance = euclidean_distance(&head, &tail);
    if distance.0 > 2 {
        if distance.2.abs() >= 1 && distance.1.abs() >= 1 {
            tail.0 += distance.1 / distance.1.abs();
            tail.1 += distance.2 / distance.2.abs();
        } else if distance.1.abs() >= 1 {
            tail.0 += distance.1 / distance.1.abs();
        } else if distance.2.abs() >= 1 {
            tail.1 += distance.2 / distance.2.abs();
        }
        visited_positions.insert(*tail);
    }
    visited_positions
}

fn euclidean_distance(p1: &(i32, i32), p2: &(i32, i32)) -> (i32, i32, i32) {
    let delta_x: i32 = (p1.0 - p2.0).try_into().unwrap();
    let delta_y: i32 = (p1.1 - p2.1).try_into().unwrap();

    let delta_x_squared = delta_x * delta_x;
    let delta_y_squared = delta_y * delta_y;

    (delta_x_squared + delta_y_squared, delta_x, delta_y)
}

fn print_move(head: &(i32, i32), tail: &(i32, i32), n: usize) {
    let mut matrix = Vec::<char>::with_capacity(n);
    matrix.fill('.');
    matrix[(head.0 * n as i32 + head.1) as usize] = 'H';
    matrix[(tail.0 * n as i32 + tail.1) as usize] = 'T';
    let mut i = 0;
    for c in matrix {
        if i % n == 0 {
            println!();
        }
        print!("{c}");
        i += 1;
    }
    println!();
}
