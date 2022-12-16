mod utils;

use std::cmp::Ordering;

use serde_json::{from_str, Value};
use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/emergency.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    let json_pairs: Vec<Vec<Value>> = input.split("\n\n").fold(Vec::new(), |mut vec, s| {
        vec.push(s.lines().fold(Vec::new(), |mut acc, l| {
            acc.push(from_str(&l).unwrap_or(Value::default()));
            acc
        }));
        vec
    });

    let mut sum = 0;
    for (i, v) in json_pairs.iter().enumerate() {
        let v1 = vec![v[0].clone()];
        let v2 = vec![v[1].clone()];
        let eq = check_list(&v1, &v2);
        match eq {
            Some(Ordering::Less) | Some(Ordering::Equal) => {
                sum += i + 1;
            }
            _ => {}
        }
    }
    println!("{sum}");
    let mut json_pairs: Vec<Value> =
        input
            .lines()
            .filter(|l| l.len() > 0)
            .fold(Vec::new(), |mut acc, l| {
                acc.push(from_str(&l).unwrap_or(Value::default()));
                acc
            });
    let divider1 = serde_json::to_value(vec![vec![2]]).unwrap();
    let divider2 = serde_json::to_value(vec![vec![6]]).unwrap();
    json_pairs.push(divider1.clone());
    json_pairs.push(divider2.clone());
    json_pairs.sort_by(|v1, v2| check_list(&vec![v1.clone()], &vec![v2.clone()]).unwrap());
    let pos1 = json_pairs
        .iter()
        .position(|e| {
            check_list(&vec![e.clone()], &vec![divider1.clone()]) == Some(Ordering::Equal)
        })
        .unwrap();
    let pos2 = json_pairs
        .iter()
        .position(|e| {
            check_list(&vec![e.clone()], &vec![divider2.clone()]) == Some(Ordering::Equal)
        })
        .unwrap();
    println!("{}", (pos1 + 1) * (pos2 + 1));
}

fn check_list(left: &Vec<Value>, right: &Vec<Value>) -> Option<Ordering> {
    for (v1, v2) in left.into_iter().zip(right.into_iter()) {
        let mut eq = None;

        match (v1, v2) {
            (Value::Array(a1), Value::Array(a2)) => {
                eq = check_list(a1, a2) /*  && a1.len() <= a2.len() */
            }
            (Value::Number(n1), Value::Number(n2)) => {
                eq = n1.as_i64().unwrap().partial_cmp(&n2.as_i64().unwrap());
            }
            (Value::Array(a), Value::Number(_)) => {
                // let mut new_vec = Vec::new();
                // new_vec.reserve(a.len());
                // for _ in 0..a.len() {
                //     new_vec.push(Value::from(n.as_i64().unwrap()));
                // }
                eq = check_list(a, &vec![v2.clone()]) /* && a.len() <= new_vec.len(); */
            }
            (Value::Number(_), Value::Array(a)) => {
                eq = check_list(&vec![v1.clone()], a) /* && vec![v1.clone()].len() <= a.len() */
            }
            _ => {}
        }
        if eq != Some(Ordering::Equal) {
            return eq;
        }
    }

    left.len().partial_cmp(&right.len())
}
