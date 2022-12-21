mod utils;
use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/encrypted.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    let decrypt_key = 811589153;
    let nums: Vec<(usize, i64)> = input
        .lines()
        .enumerate()
        .map(|(i, n)| (i, n.parse::<i64>().unwrap_or(0)))
        .collect();
    let res = shift_array(&nums, 1);
    println!("Part 1: {res}");
    let nums: Vec<(usize, i64)> = input
        .lines()
        .enumerate()
        .map(|(i, n)| (i, n.parse::<i64>().unwrap_or(0) * decrypt_key))
        .collect();
    let res = shift_array(&nums, 10);
    println!("Part 2: {res}");
}

fn shift_array(nums: &Vec<(usize, i64)>, mixes: usize) -> i64 {
    let mut nums_clone = nums.clone();
    for _ in 0..mixes {
        for i in 0..nums.len() {
            let num = nums[i];
            let i_cur: usize = nums_clone.iter().position(|(j, _)| *j == num.0).unwrap();
            let shift = (i_cur as i64 + num.1).rem_euclid(nums.len() as i64 - 1) as usize;
            nums_clone.remove(i_cur);
            nums_clone.insert(shift, num);
        }
    }

    let zero = nums_clone.iter().position(|(_, n)| *n == 0).unwrap();

    let n1 = nums_clone[(zero + 1000) % nums_clone.len()];
    let n2 = nums_clone[(zero + 2000) % nums_clone.len()];
    let n3 = nums_clone[(zero + 3000) % nums_clone.len()];
    n1.1 + n2.1 + n3.1
}
