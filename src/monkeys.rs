mod utils;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::f32::MIN_POSITIVE;

use utils::utils::process_lines;
use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/monkeys.txt";
const ERROR_MESSAGE: &str = "Error reading input";

#[derive(Debug, Clone)]
pub struct Monkey {
    pub id: u32,
    pub items: Vec<u128>,
    pub operation: String,
    pub divisor: u128,
    pub true_monkey: u32,
    pub false_monkey: u32,
}

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    let monkeys = parse_monkeys(&input);
    println!("{}", monkeys.1);

    let res1 = do_operation(monkeys.0.clone(), 20, 3, monkeys.1);
    println!("{}", res1[0] * res1[1]);

    let res1 = do_operation(monkeys.0.clone(), 10000, 1, monkeys.1);
    println!("{}", res1[0] * res1[1]);
}

fn do_operation(
    mut monkeys: HashMap<u32, Monkey>,
    rounds: u32,
    worried_level: u128,
    normalizer: u128,
) -> Vec<u128> {
    let mut res = vec![0; monkeys.len()];
    println!("{} {}", monkeys.len(), res.len());
    for _ in 0..rounds {
        for monkey in 0..monkeys.len() {
            let mut moves = HashMap::<u32, Vec<u128>>::new();

            {
                let monkey_index = monkey as u32;
                let monkey = monkeys.get_mut(&monkey_index).unwrap();
                for item in monkey.items.iter() {
                    let item = operation(&monkey.operation, *item, worried_level, normalizer);
                    let to_monkey = test(
                        item,
                        monkey.divisor,
                        monkey.true_monkey,
                        monkey.false_monkey,
                    );
                    match moves.get_mut(&to_monkey) {
                        Some(val) => val.push(item),
                        None => {
                            let mut vec = Vec::new();
                            vec.push(item);
                            moves.insert(to_monkey, vec);
                        }
                    }
                    res[monkey_index as usize] += 1
                }

                monkey.items.clear();
            }
            for mov in moves {
                for item in mov.1 {
                    monkeys.get_mut(&mov.0).unwrap().items.push(item);
                }
            }
        }
        // println!("{:?}", monkeys);
    }
    res.sort();
    res.reverse();
    res
}

fn operation(operation: &String, old: u128, worried_level: u128, normalizer: u128) -> u128 {
    let mut splits = operation.split_whitespace();
    // let first_operand = splits.next().unwrap_()
    splits.next();
    match splits.next().unwrap_or("+") {
        "+" => {
            let second_operand = splits.next().unwrap_or("0").trim();
            let second_operand = match second_operand {
                "old" => old,
                _ => second_operand.parse::<u128>().unwrap_or(0),
            };
            ((old + second_operand) / worried_level) % normalizer
        }
        "*" => {
            let second_operand = splits.next().unwrap_or("0").trim();
            let second_operand = match second_operand {
                "old" => old,
                _ => second_operand.parse::<u128>().unwrap_or(0),
            };
            ((old * second_operand) / worried_level) % normalizer
        }
        _ => old,
    }
}

fn test(val: u128, divisor: u128, true_monkey: u32, false_monkey: u32) -> u32 {
    if val % divisor as u128 == 0 {
        true_monkey
    } else {
        false_monkey
    }
}

pub fn parse_monkeys(input: &String) -> (HashMap<u32, Monkey>, u128) {
    input
        .split("\n\n")
        .fold((HashMap::<u32, Monkey>::new(), 1u128), |mut acc, monkey| {
            let monkey = parse_monkey(&monkey.to_string(), acc.1);
            acc.0.insert(monkey.0.id, monkey.0);
            acc.1 = monkey.1;
            acc
        })
}

fn parse_monkey(input: &String, old_divider: u128) -> (Monkey, u128) {
    let mut lines = input.lines();
    let mut divider = old_divider;
    let mut splits = lines.next().unwrap_or("").split_whitespace();
    splits.next();
    let mut items: Vec<u128> = Vec::new();
    let mut operation = "";
    let mut divisor = 1u128;
    let mut destiny_monkeys = (0u32, 0u32);
    let id = splits
        .next()
        .unwrap()
        .replace(":", "")
        .parse::<u32>()
        .unwrap();
    for line in lines {
        let split_index = line.find(":");
        let splits = line.split_at(split_index.unwrap());
        match splits.0.trim() {
            "Starting items" => {
                items = splits
                    .1
                    .replace(":", "")
                    .split(",")
                    .fold(Vec::new(), |mut acc, s| {
                        let val = s.trim().parse::<u128>().unwrap_or(0);
                        acc.push(val);
                        acc
                    });
            }
            "Operation" => {
                let split_index = line.find("old");
                let splits = line.split_at(split_index.unwrap());
                operation = splits.1.trim();
            }
            "Test" => {
                let split_index = line.find("by");
                let splits = line.split_at(split_index.unwrap());
                divisor = splits.1.replace("by", "").trim().parse().unwrap_or(1);
                divider = divider * divisor;
            }
            "If true" => {
                let split_index = line.find("monkey");
                let splits = line.split_at(split_index.unwrap());
                destiny_monkeys.0 = splits.1.replace("monkey", "").trim().parse().unwrap_or(1);
            }
            "If false" => {
                let split_index = line.find("monkey");
                let splits = line.split_at(split_index.unwrap());
                destiny_monkeys.1 = splits.1.replace("monkey", "").trim().parse().unwrap_or(1);
            }
            _ => {}
        }
    }
    (
        Monkey {
            id,
            items,
            operation: operation.to_string(),
            divisor,
            true_monkey: destiny_monkeys.0,
            false_monkey: destiny_monkeys.1,
        },
        divider,
    )
}
