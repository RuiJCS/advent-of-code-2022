mod utils;
use utils::utils::process_lines;
use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/sack.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    let mut result = process_lines(&input, count_elf_calories);
    println!("{}", result.iter().sum::<u32>());
    result = badges(&input);
    println!("{:?}", result.iter().sum::<u32>());
}

fn count_elf_calories(line: &str) -> u32 {
    let mut res = 0u32;
    let split_at = if line.len() % 2 == 0 {
        line.len() / 2
    } else {
        line.len() / 2 + 1
    };
    let splits = line.split_at(split_at);
    let mut first_split = splits.0.chars().collect::<Vec<char>>();
    first_split.sort();
    first_split.dedup();
    for c in first_split {
        if splits.1.find(c).is_some() {
            if c.is_uppercase() {
                res += c as u32 - 'A' as u32 + 27;
            } else {
                res += c as u32 - 'a' as u32 + 1;
            }
        }
    }
    res
}

fn badges(file: &String) -> Vec<u32> {
    let mut res = Vec::<u32>::new();
    let mut group = Vec::<String>::new();
    for line in file.lines() {
        group.push(String::from(line));
        if group.len() == 3 {
            let first = &group[0];
            let second = &group[1];
            let third = &group[2];
            let mut repeated_chars = Vec::<char>::new();
            let mut first_split = first.chars().collect::<Vec<char>>();
            first_split.sort();
            first_split.dedup();
            for c in first_split {
                if second.find(c).is_some() {
                    repeated_chars.push(c);
                    if third.find(c).is_some() {
                        if c.is_uppercase() {
                            res.push(c as u32 - 'A' as u32 + 27);
                        } else {
                            res.push(c as u32 - 'a' as u32 + 1);
                        }
                    }
                }
            }
            group.clear();
        }
    }
    res
}
