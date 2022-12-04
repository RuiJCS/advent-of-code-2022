mod utils;
use std::default;

use utils::utils::process_lines;
use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/rps.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    let mut result = normal_rps(&input, score_each_game);
    println!("{}", result.iter().sum::<u32>());
    result = normal_rps(&input, score_each_game_rigged);
    println!("{:?}", result.iter().sum::<u32>());
}

fn real_score(val: u32) -> u32 {
    match val {
        1 => 1,
        2 => 3,
        3 => 2,
        _ => 0,
    }
}

fn score_each_game(line: &str) -> u32 {
    let mut res = 0u32;
    let mut chars = line.split_whitespace();
    let oponent = chars
        .next()
        .unwrap()
        .chars()
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap();
    let my = chars
        .next()
        .unwrap()
        .chars()
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap();
    if my == 3 && oponent == 1 {
        res = real_score(my) + 6;
    } else if my == 1 && oponent == 3 {
        res = real_score(my);
    } else if my > oponent {
        res = real_score(my);
    } else if my == oponent {
        res = real_score(my) + 3;
    } else if my < oponent {
        res = real_score(my) + 6;
    }
    res
}

fn score_each_game_rigged(line: &str) -> u32 {
    let mut res = 0u32;
    let parsed_line = line
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    let mut chars = parsed_line.chars();
    let oponent = chars.next().unwrap().to_digit(10).unwrap();
    let my = chars.next().unwrap().to_digit(10).unwrap();
    res = match my {
        1 => match oponent {
            3 => real_score(1),
            _ => real_score(oponent + 1),
        },
        3 => real_score(oponent) + 3,
        2 => match oponent {
            1 => real_score(3) + 6,
            _ => real_score(oponent - 1) + 6,
        },
        _ => 0,
    };
    res
}

fn normal_rps<F>(file: &String, mut f: F) -> Vec<u32>
where
    F: FnMut(&str) -> u32,
{
    let final_file = file
        .chars()
        .map(|c| match c {
            'A' | 'X' => '1', // rock | lose
            'B' | 'Y' => '3', // paper | draw
            'C' | 'Z' => '2', // scisor | win
            _ => c,
        })
        .collect::<String>();
    process_lines(&final_file, f)
}
