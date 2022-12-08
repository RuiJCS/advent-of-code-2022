mod utils;
use std::{collections::{HashMap, VecDeque, vec_deque, btree_map::Values}, path};

use utils::utils::read_file;

// const DEPTH_FILE_NAME: &str = "inputs/test_moves.txt";
const DEPTH_FILE_NAME: &str = "inputs/commands.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    let dirs = parse_commands(&input);
    println!("{}", get_smaller_than(&dirs, 100000));
    println!("{}", get_smaller_to_free(&dirs, 70000000));
}

fn parse_commands(input:&String) -> HashMap<String,Vec<(u32,String)>> {
    let mut curr_path = "".to_string();
    let mut dirs_size = HashMap::<String,Vec<(u32,String)>>::new();

    for line in input.lines() {
        if line.contains("$ cd")  {
            let mut splits = line.split_whitespace();
            splits.next();
            splits.next();
            match splits.next() {
                Some(path) => {
                    match path {
                        ".." => {
                            let pos = curr_path.chars().count() - curr_path.chars().rev().position(|c| c == '/').unwrap() - 1;
                            curr_path.truncate(pos);
                        },
                        "/" => {

                        }
                        _ => {
                            curr_path = curr_path.clone() + "/" + path;
                        }
                    }
                },
                None => {}
            }
        } else if line.chars().next().unwrap().is_digit(10) {
            let mut splits = line.split_whitespace();
            let size = splits.next().unwrap().parse::<u32>().unwrap();
            let file_name = splits.next().unwrap().to_string();
            match dirs_size.get_mut(&curr_path) {
                Some(vec) => {
                    vec.push((size,file_name));
                },
                None => {
                    let mut vec = Vec::<(u32,String)>::new();
                    vec.push((size,file_name));
                    dirs_size.insert(curr_path.clone(), vec);
                }
            }
        }
    }
    
    dirs_size
}

fn get_smaller_than(input:&HashMap<String,Vec<(u32,String)>>, N: u32) -> u32 {
    
    let mut size = HashMap::<String,u32>::new();
    for kv in input {
        for file in kv.1 {
            let paths = kv.0.split("/");
            let mut cur_path = "".to_string();
            for path  in paths {
                cur_path = cur_path.clone() + "/" + path;
                match size.get_mut(&cur_path) {
                    Some(val) => {
                        *val = *val + file.0;
                    },
                    None => {
                        size.insert(cur_path.clone(), file.0);
                    }
                }
            }
        }
    }
    size.iter().filter(|kv| *kv.1 < N).fold(0,|acc, kv| kv.1 + acc )
}

fn get_smaller_to_free(input:&HashMap<String,Vec<(u32,String)>>, N: u32) -> u32 {
    let mut size = HashMap::<String,u32>::new();
    for kv in input {
        for file in kv.1 {
            let paths = kv.0.split("/");
            let mut cur_path = "".to_string();
            for path  in paths {
                cur_path = cur_path.clone() + "/" + path;
                match size.get_mut(&cur_path) {
                    Some(val) => {
                        *val = *val + file.0;
                    },
                    None => {
                        size.insert(cur_path.clone(), file.0);
                    }
                }
            }
        }
    }
    println!("{:?}", size);
    let free_space_needed = N - size.get("/").unwrap();

    let vec= size.values().collect::<Vec<&u32>>();
    let mut vec: Vec<&&u32> = vec.iter().filter(|v| ***v > 30000000 - free_space_needed).collect();
    vec.sort();
    
    println!("{}",vec.first().unwrap());

    ***vec.first().unwrap()
}