mod utils;
use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/calories.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    let mut result = max(&input, count_elf_calories, 1);
    println!("{:?}", result);
    result = max(&input, count_elf_calories, 3);
    println!("{:?}", result);
}

fn count_elf_calories(lines: &str) -> u32 {
    let mut res = 0u32;
    for line in lines.lines() {
        let line_val = line.parse::<u32>();
        res += if let Result::Ok(val) = line_val {
            val
        } else {
            0
        }
    }
    res
}

fn max<F>(file: &String, mut f: F, final_size: usize) -> Vec<u32>
where
    F: FnMut(&str) -> u32,
{
    let mut res = vec![0, 0, 0];
    let elfs_lines = file.rsplit("\n\n");
    for elf_list in elfs_lines {
        // println!("{}\n============================", elf_list);
        let val = count_elf_calories(elf_list);
        res.push(val);
    }
    res.sort();
    res.reverse();
    res.truncate(final_size);
    res
}
