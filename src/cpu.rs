mod utils;
use std::collections::HashMap;
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};
use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/cpu.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    let keys = [20, 60, 100, 140, 180, 220];
    let clocks = process_clocks(&input);
    let mut res = 0;
    for key in keys {
        println!("{} {}", key, clocks.get(&(key)).unwrap_or(&0));
        // println!("{} {}", key - 1, clocks.get(&(key - 1)).unwrap_or(&0));
        // println!("{} {}", key + 1, clocks.get(&(key + 1)).unwrap_or(&0));
        res += *clocks.get(&(key)).unwrap_or(&0) * key as i32;
    }
    println!("{res}");
    process_crt(&input);
}

fn process_crt(input: &String) {
    input
        .lines()
        .enumerate()
        .fold((1, 1, ['.'; 40]), |mut acc, line| {
            let mut splits = line.1.split_whitespace();
            match splits.next() {
                Some(s) => match s {
                    "noop" => {
                        draw_srpite(acc.0, acc.1, &mut acc.2);
                        acc.0 += 1;
                    }
                    "addx" => {
                        let val = splits.next().unwrap().parse::<i32>();
                        draw_srpite(acc.0, acc.1, &mut acc.2);
                        acc.0 += 1;
                        draw_srpite(acc.0, acc.1, &mut acc.2);
                        acc.1 += val.unwrap_or(0);
                        acc.0 += 1;
                    }
                    _ => {}
                },
                None => {}
            };
            // println!("{}", acc.1);
            acc
        });
}

fn process_clocks(input: &String) -> HashMap<u32, i32> {
    input
        .lines()
        .enumerate()
        .fold((HashMap::new(), 1, 1), |mut acc, line| {
            let mut splits = line.1.split_whitespace();
            match splits.next() {
                Some(s) => match s {
                    "noop" => {
                        acc.1 += 1;
                        insert_add(&mut acc.0, acc.1, acc.2);
                    }
                    "addx" => {
                        let val = splits.next().unwrap().parse::<i32>();
                        acc.1 += 1;
                        insert_add(&mut acc.0, acc.1, acc.2);
                        acc.2 += val.unwrap_or(0);
                        acc.1 += 1;
                        insert_add(&mut acc.0, acc.1, acc.2);
                    }
                    _ => {}
                },
                None => {}
            };
            // println!("{}", acc.1);
            acc
        })
        .0
}

fn insert_add(acc: &mut HashMap<u32, i32>, key: u32, val: i32) {
    let get = acc.get(&key);
    acc.insert(key, val + get.unwrap_or(&0));
}

fn calculate_draw_index(index: usize) -> usize {
    // println!("{}", index % 40);
    index % 40
}

fn calculate_sprite(index: usize) -> [char; 40] {
    let index = if index == 0 {
        1
    } else if index >= 39 {
        38
    } else {
        index
    };
    let mut sprite = ['.'; 40];
    sprite[index - 1] = '#';
    sprite[index] = '#';
    sprite[index + 1] = '#';
    sprite
}

fn draw_srpite(draw_index: usize, sprite_index: i32, curr_drawing: &mut [char; 40]) {
    if draw_index % 40 == 0 {
        println!();
        *curr_drawing = ['.'; 40];
    }
    let draw_index = calculate_draw_index(draw_index - 1);
    let sprite_index = calculate_draw_index(sprite_index as usize);
    let sprite = calculate_sprite(sprite_index);
    curr_drawing[draw_index] = sprite[draw_index];
    print!("\r{}", curr_drawing.iter().collect::<String>());
    stdout().flush().unwrap();
}
