mod utils;
use utils::utils::process_lines;
use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/cleaning.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    let mut result = process_lines(&input, total_overlaping);
    println!("{}", result.iter().sum::<u32>());
    result = process_lines(&input, partial_overlaping);
    println!("{}", result.iter().sum::<u32>());
}

fn prepare_pairings(line: &str) -> ((u32, u32), (u32, u32)) {
    let mut pairs = line.split(",");
    let mut pair1_string = pairs.next().unwrap().split("-");
    let mut pair2_string = pairs.next().unwrap().split("-");
    let pair1 = (
        pair1_string.next().unwrap().parse::<u32>().unwrap(),
        pair1_string.next().unwrap().parse::<u32>().unwrap(),
    );
    let pair2 = (
        pair2_string.next().unwrap().parse::<u32>().unwrap(),
        pair2_string.next().unwrap().parse::<u32>().unwrap(),
    );
    (pair1, pair2)
}

fn total_overlaping(line: &str) -> u32 {
    let mut res = 0u32;
    let (pair1, pair2) = prepare_pairings(line);
    if (pair1.0 <= pair2.0 && pair1.1 >= pair2.1) || (pair1.0 >= pair2.0 && pair1.1 <= pair2.1) {
        res = 1;
    }
    res
}

fn partial_overlaping(line: &str) -> u32 {
    let mut res = 0u32;
    let (pair1, pair2) = prepare_pairings(line);
    if (pair1.1 <= pair2.1 && pair1.1 >= pair2.0)
        || (pair1.0 >= pair2.0 && pair1.0 <= pair2.1)
        || (pair2.1 <= pair1.1 && pair2.1 >= pair1.0)
        || (pair2.0 >= pair1.0 && pair2.0 <= pair1.1)
    {
        res = 1;
    }
    res
}
