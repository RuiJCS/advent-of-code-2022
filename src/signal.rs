mod utils;
use std::collections::VecDeque;

use utils::utils::process_lines;
use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/signal.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    let result = process_different_four_chars(&input, 4);
    println!("{result}");

    let result = process_different_four_chars(&input, 14);
    println!("{result}");
}

fn process_different_four_chars(input: &String, n: usize) -> u32 {
    let mut res = 0u32;
    let mut char_sequence = VecDeque::new();
    for c in input.chars() {
        res += 1;
        char_sequence.push_back(c);
        if char_sequence.len() == n {
            let mut vec: Vec<char> = char_sequence.clone().into();
            vec.sort();
            vec.dedup();
            if vec.len() == n {
                break;
            }
            char_sequence.pop_front();
        }
    }
    res
}
