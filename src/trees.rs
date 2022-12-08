mod utils;

use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/trees.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    // let chars = input.lines().fold(0,|acc, s| acc + s.len());
    let matrix: Vec<u32> = input.lines().fold(Vec::new(), |mut acc, s| {
        s.chars().for_each(|c| acc.push(c.to_digit(10).unwrap()));
        acc
    });
    let row_size = input.lines().count();
    let matrix: Vec<Vec<u32>> = matrix.chunks(row_size).map(|c| c.into()).collect();
    let mut res1 = 0;
    let mut res2 = 0;

    for i in 0..row_size {
        for j in 0..row_size {
            let val = matrix[i][j];
            
            // Part 1 
            res1 += if i == 0 || i == row_size-1 || j == 0 || j == row_size-1{
                    1
                } else {
                    let mut is_visible;
                    is_visible = (line_of_sight_horizontal(&matrix, val, 0, i, j) == 1) as i32;
                    if is_visible == 0 {
                        is_visible = (line_of_sight_horizontal(&matrix, val, i+1, row_size, j) == 1) as i32;
                    }

                    if is_visible == 0 {
                        is_visible = (line_of_sight_vertical(&matrix, val, 0, j, i) == 1) as i32;
                    }
                    if is_visible == 0 {
                        is_visible = (line_of_sight_vertical(&matrix, val, j+1, row_size, i) == 1) as i32;

                    }
                    is_visible
                };

                // Part 2
                let max  = {
                            let mut score = 1;
                            score *= walk_vertical(&matrix, val, 0, i, j);
                            score *= walk_vertical(&matrix, val, i+1, row_size, j);
                            score *= walk_horizontal(&matrix, val, 0, j, i);
                            score *= walk_horizontal(&matrix, val, j+1, row_size, i);
                            score
                        };
                        if max > res2 {
                            println!("{i} {j} {val} {res2}");
                            res2 = max;
                        }
        }
    }
    println!("{res1}");
    println!("{res2}");

    
}

fn line_of_sight_horizontal(matrix: &Vec<Vec<u32>>, val: u32, start: usize, end: usize, i: usize) -> i32 {
    let mut aux = 1;
    for x in start..end {
        if matrix[x][i] >= val {
            aux = 0;
            break;
        }
    }
    aux
}

fn line_of_sight_vertical(matrix: &Vec<Vec<u32>>, val: u32, start: usize, end: usize, i: usize) -> i32 {
    let mut aux = 1;
    for x in start..end {
        if matrix[i][x] >= val {
            aux = 0;
            break;
        }
    }
    aux
}

fn walk_horizontal(matrix: &Vec<Vec<u32>>, val: u32, start: usize, end: usize, i: usize) -> u32 {
    let mut viewing_distance = 0;
    for x in start..end {
        if x > start || x < end {
            viewing_distance += 1;
            if matrix[i][x] >= val && start > 0 {
                break;
            } else if matrix[i][x] >= val {
                viewing_distance = 1;
            }
        }
    }
    viewing_distance

}

fn walk_vertical(matrix: &Vec<Vec<u32>>, val: u32, start: usize, end: usize, i: usize) -> u32 {
    let mut viewing_distance = 0;
    for x in start..end {
        if x > start || x < end {
            viewing_distance += 1;
            if matrix[x][i] >= val && start > 0 {
                break;
            } else if matrix[x][i] >= val {
                viewing_distance = 1;
            }
        }
    }
    viewing_distance

}