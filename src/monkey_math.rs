mod utils;
use std::collections::HashMap;

use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/monkey_math.txt";
const ERROR_MESSAGE: &str = "Error reading input";

#[derive(Clone)]
enum MonkeyMath {
    Math(String, String, String),
    Number(i64),
}

#[derive(PartialEq, Eq)]
enum Side {
    Right,
    Left,
    PANIC,
}

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    let speach: HashMap<String, MonkeyMath> =
        input.lines().fold(HashMap::new(), |mut speach, s| {
            let mm = parse_math(&s.to_string());
            speach.insert(mm.0, mm.1);
            speach
        });
    println!("{}", do_math(&speach, &"root".to_string()));
    println!(
        "{}",
        do_monkey_elephant_human_math(&speach, &"root".to_string())
    );
}

fn parse_math(s: &String) -> (String, MonkeyMath) {
    let mut splits = s.split(":");
    let monkey = splits.next().unwrap().to_string();
    let mm = splits.next().unwrap().trim();
    match mm.parse::<i64>() {
        Ok(num) => (monkey, MonkeyMath::Number(num)),
        Err(_) => {
            let mut splits = mm.trim().split_whitespace();
            (
                monkey,
                MonkeyMath::Math(
                    splits.next().unwrap().to_string(),
                    splits.next().unwrap().to_string(),
                    splits.next().unwrap().to_string(),
                ),
            )
        }
    }
}

fn do_math(speach: &HashMap<String, MonkeyMath>, position: &String) -> i64 {
    match speach[position].clone() {
        MonkeyMath::Math(opl, op, opr) => {
            let opl = do_math(speach, &opl);
            let opr = do_math(speach, &opr);
            match op.as_str() {
                "*" => opl * opr,
                "+" => opl + opr,
                "-" => opl - opr,
                "/" => opl / opr,
                _ => 0,
            }
        }
        MonkeyMath::Number(num) => num,
    }
}

fn do_monkey_elephant_human_math(speach: &HashMap<String, MonkeyMath>, position: &String) -> i64 {
    let root = speach[position].clone();
    let human_side = {
        let mut side = Side::PANIC;
        let mut key = "humn";
        while let Some(monkey) = speach.get(&key.to_string()) {
            let aux = &speach
                .iter()
                .filter(|(k, v)| match v {
                    MonkeyMath::Math(m1, _, m2) => m1 == key || m2 == key,
                    MonkeyMath::Number(_) => false,
                })
                .map(|(k, _)| k.clone())
                .collect::<Vec<String>>()[0];
            if aux == position {
                match root {
                    MonkeyMath::Math(ref l, _, _) => {
                        if l == key {
                            side = Side::Left;
                        } else {
                            side = Side::Right;
                        }
                    }
                    MonkeyMath::Number(_) => side = Side::PANIC,
                }
                key = ""
            } else {
                key = ""
            }
        }
        side
    };
    let non_human_side = {
        let (l, r) = match root {
            MonkeyMath::Math(l, _, r) => (l, r),
            MonkeyMath::Number(_) => ("".to_string(), "".to_string()),
        };
        if human_side == Side::Left {
            do_math(speach, &r)
        } else {
            do_math(speach, &l)
        }
    };
    non_human_side
}
