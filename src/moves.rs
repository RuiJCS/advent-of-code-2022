mod utils;
use std::{collections::{HashMap, VecDeque, vec_deque, btree_map::Values}};

use utils::utils::read_file;

// const DEPTH_FILE_NAME: &str = "inputs/test_moves.txt";
const DEPTH_FILE_NAME: &str = "inputs/moves.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    let input_binding = input.clone();
    let mut split = input_binding.split("\n\n");
    let stacks = parse_lines_hash(&split.next().unwrap().into(), parse_stacks);
    
    let moves = parse_lines(&split.next().unwrap().into(), parse_moves);
    println!("{:?}", moves);
    let res = make_moves(&stacks, &moves);
    
    for stack in 1..(res.len()+1) as u32 {
        let stack = res.get(&stack);
        match stack {
            Some(stack) => {
                print!("{}", stack.front().unwrap());
            },
                None => {
                }
        }
    }

    println!("");
    let res = make_moves_2(&stacks, &moves);
    for stack in 1..(res.len()+1) as u32 {
        let stack = res.get(&stack);
        match stack {
            Some(stack) => {
                print!("{}", stack.front().unwrap());
            },
            None => {
            }
        }
    }
}

fn make_moves(stacks :&HashMap<u32, VecDeque<String>>, moves: &Vec<Vec<u32>>) -> HashMap<u32, VecDeque<String>> {
    let mut res = stacks.clone();
    for instruction in moves {
        let (quantity, from, to) = (instruction[0], instruction[1], instruction[2]);
        for i in 0..quantity {
            if let Some(val) = res.get_mut(&from) {
                if let Some(string) = val.pop_front() {
                    if let Some(stack) = res.get_mut(&to) {
                        stack.push_front(string);
                    } else {}
                }else {}
            }else {};
        }
    }
    res
}

fn make_moves_2(stacks :&HashMap<u32, VecDeque<String>>, moves: &Vec<Vec<u32>>) -> HashMap<u32, VecDeque<String>> {
    let mut res = stacks.clone();
    for instruction in moves {
        let (quantity, from, to) = (instruction[0], instruction[1], instruction[2]);
        let mut vals = Vec::new();
        for i in 0..quantity {
            if let Some(val) = res.get_mut(&from) {
                if let Some(string) = val.pop_front() {
                    vals.push(string);
                }else {}
            }else {};
        }
        vals.reverse();
        if let Some(stack) = res.get_mut(&to) {
            for string in vals {
                stack.push_front(string);
            }
        } else {}
    }
    res
}

fn parse_stacks(stack_line: &str) -> HashMap<u32, String> {
    let mut res = HashMap::new();
    let mut i = 1;
    for item in stack_line.chars().collect::<Vec<char>>().chunks(4) {
        let item: String = item.iter().filter(|c| c.is_alphabetic()).collect();
        if !item.is_empty() {
            res.insert(i, item);
        }
        i+=1;
    }
    res
}

fn parse_moves(move_line: &str) -> Vec<u32> {
    let mut res =Vec::new();
    for item in move_line.split_whitespace() {
        let mut item: String = item.chars().filter(|c| c.is_digit(10)).collect();
        item.retain(|c| !c.is_whitespace());
        if !item.is_empty() {
            if item.len() >1 {
                println!("{} {}", item,item.parse::<u32>().unwrap());
            }
            let val = item.parse::<u32>().unwrap();
            res.push(val);
        }
    }
    res
}

fn parse_lines<F, T>(file: &String, mut f: F) -> Vec<T>
where
    F: FnMut(&str) -> T,
{
    let mut res = Vec::<T>::new();
    for elf_list in file.lines() {
        let val = f(elf_list);
        res.push(val);
    }
    res
}

fn parse_lines_hash<F>(file: &String, mut f: F) -> HashMap<u32,VecDeque<String>>
where
    F: FnMut(&str) -> HashMap<u32,String>,
{
    let mut res: HashMap<u32, VecDeque<String>> = HashMap::new();
    for elf_list in file.lines() {
        let val = f(elf_list);
        for (key,value) in val.iter() {
            match res.get_mut(key) {
                Some(val) => val.push_back(value.clone()),
                None => {
                    println!("{key}");
                    let mut vec = VecDeque::new();
                    vec.push_back(value.to_string());
                    res.insert(*key, vec);
                }
            }
        }
        println!("{:?}", res);
        
    }
    
    res
}