mod utils;

use std::f32::consts::PI;

use utils::utils::read_file;

const DEPTH_FILE_NAME: &str = "inputs/password.txt";
const ERROR_MESSAGE: &str = "Error reading input";

fn main() {
    let input = read_file(DEPTH_FILE_NAME, ERROR_MESSAGE);
    let mut splits = input.split("\n\n");
    let map_lines = splits.next().unwrap();
    let num_lines = map_lines.lines().count();
    let max_line = map_lines.lines().map(|l| l.len()).max().unwrap();
    let map: Vec<Vec<char>> = map_lines.lines().fold(Vec::new(), |mut acc, s| {
        let mut line = Vec::new();
        line.resize_with(max_line, || ' ');
        acc.push(s.chars().enumerate().fold(line, |mut line, c| {
            line.insert(c.0, c.1);
            line
        }));
        acc
    });
    // for v in map.iter() {
    //     for c in v.iter() {
    //         print!("{c:1}");
    //     }
    //     println!();
    // }
    let indications = splits.next().unwrap();
    let indications: Vec<String> = indications
        .replace("R", " R ")
        .replace("L", " L ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let path = draw_path(&map, &indications, max_line, num_lines);
    // for v in path.iter() {
    //     for c in v.iter() {
    //         print!("{c:1}");
    //     }
    //     println!();
    // }
}

fn draw_path(
    map: &Vec<Vec<char>>,
    indications: &Vec<String>,
    max_line: usize,
    num_lines: usize,
) -> Vec<Vec<char>> {
    let mut drawing_map = map.clone();
    let mut direction_angle = 0f32;
    let mut position = (map[0].iter().position(|c| *c == '.').unwrap() as i32, 0i32);
    drawing_map[position.1 as usize][position.0 as usize] = '>';
    for c in indications {
        match c.parse::<u32>() {
            Ok(num) => {
                for _ in 0..num {
                    let direction = (direction_angle.cos() as i32, direction_angle.sin() as i32);
                    let fp = (position.0 + direction.0, position.1 + direction.1);
                    // println!("{position:?} {fp:?}");
                    match direction {
                        (0, 1) => {
                            if fp.1 >= num_lines as i32 {
                                let new_line = end_or_whitespace_vertical(map, 1, fp);
                                if map[new_line][fp.0 as usize] == '#' {
                                    break;
                                }
                                drawing_map[new_line][fp.0 as usize] = 'V';
                                position = (fp.0, new_line as i32);
                            } else if map[fp.1 as usize][fp.0 as usize].is_ascii_whitespace() {
                                let new_line = end_or_whitespace_vertical(map, 1, fp);
                                if map[new_line][fp.0 as usize] == '#' {
                                    break;
                                }
                                drawing_map[new_line][fp.0 as usize] = 'V';
                                position = (fp.0, new_line as i32);
                            } else {
                                if map[fp.1 as usize][fp.0 as usize] == '#' {
                                    break;
                                }
                                drawing_map[fp.1 as usize][fp.0 as usize] = 'V';
                                position = fp;
                            }
                        }
                        (0, -1) => {
                            if fp.1 < 0 {
                                let new_line = end_or_whitespace_vertical(map, -1, fp);
                                if map[new_line][fp.0 as usize] == '#' {
                                    break;
                                }
                                drawing_map[new_line][fp.0 as usize] = '^';
                                position = (fp.0, new_line as i32);
                            } else if map[fp.1 as usize][fp.0 as usize].is_ascii_whitespace() {
                                let new_line = end_or_whitespace_vertical(map, -1, fp);
                                if map[new_line][fp.0 as usize] == '#' {
                                    break;
                                }
                                drawing_map[new_line][fp.0 as usize] = '^';
                                position = (fp.0, new_line as i32);
                            } else {
                                if map[fp.1 as usize][fp.0 as usize] == '#' {
                                    break;
                                }
                                drawing_map[fp.1 as usize][fp.0 as usize] = '^';
                                position = fp;
                            }
                        }
                        (1, 0) => {
                            if fp.0 >= max_line as i32 {
                                let new_column =
                                    end_or_whitespace_horizontal(&map[fp.1 as usize], 1);
                                println!("1 {new_column}");
                                if map[fp.1 as usize][new_column] == '#' {
                                    break;
                                }
                                drawing_map[fp.1 as usize][new_column] = '>';
                                position = (new_column as i32, fp.1);
                            } else if map[fp.1 as usize][fp.0 as usize].is_ascii_whitespace() {
                                let new_column =
                                    end_or_whitespace_horizontal(&map[fp.1 as usize], 1);
                                if map[fp.1 as usize][new_column] == '#' {
                                    break;
                                }
                                drawing_map[fp.1 as usize][new_column] = '>';
                                position = (new_column as i32, fp.1);
                            } else {
                                if map[fp.1 as usize][fp.0 as usize] == '#' {
                                    break;
                                }
                                drawing_map[fp.1 as usize][fp.0 as usize] = '>';
                                position = fp;
                            }
                        }

                        (-1, 0) => {
                            if fp.0 < 0 {
                                let new_column =
                                    end_or_whitespace_horizontal(&map[fp.1 as usize], -1);
                                println!("-1 {new_column}");
                                if map[fp.1 as usize][new_column] == '#' {
                                    break;
                                }
                                drawing_map[fp.1 as usize][new_column] = '<';
                                position = (new_column as i32, fp.1);
                            } else if map[fp.1 as usize][fp.0 as usize].is_ascii_whitespace() {
                                let new_column =
                                    end_or_whitespace_horizontal(&map[fp.1 as usize], -1);
                                if map[fp.1 as usize][new_column] == '#' {
                                    break;
                                }
                                drawing_map[fp.1 as usize][new_column] = '<';
                                position = (new_column as i32, fp.1);
                            } else {
                                if map[fp.1 as usize][fp.0 as usize] == '#' {
                                    break;
                                }
                                drawing_map[fp.1 as usize][fp.0 as usize] = '<';
                                position = fp;
                            }
                        }
                        _ => {}
                    }
                }
            }
            Err(_) => {
                direction_angle += {
                    match c.as_str() {
                        "R" => PI / 2f32,
                        "L" => -PI / 2f32,
                        _ => 0f32,
                    }
                }
            }
        }
    }
    let direction = 0 * (direction_angle.cos() == 1f32) as i32
        + 1 * (direction_angle.sin() == 1f32) as i32
        + 2 * (direction_angle.cos() == -1f32) as i32
        + 3 * (direction_angle.sin() == -1f32) as i32;
    println!(
        "{} {} {} {} {}",
        position.1,
        position.0,
        direction_angle,
        direction,
        1000 * (position.1 + 1) + 4 * (position.0 + 1) + direction
    );
    drawing_map
}

fn end_or_whitespace_vertical(map: &Vec<Vec<char>>, direction: i32, position: (i32, i32)) -> usize {
    if direction == 1 {
        println!("{position:?}");
        *map.iter()
            .enumerate()
            .filter(|(_, c)| c[position.0 as usize] == '.' || c[position.0 as usize] == '#')
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
            .first()
            .unwrap()
    } else {
        let mut local_map = map.clone();
        local_map.reverse();
        local_map.len()
            - *local_map
                .iter()
                .enumerate()
                .filter(|(_, c)| c[position.0 as usize] == '.' || c[position.0 as usize] == '#')
                .map(|(i, _)| i)
                .collect::<Vec<usize>>()
                .first()
                .unwrap()
            - 1
    }
}

fn end_or_whitespace_horizontal(map: &Vec<char>, direction: i32) -> usize {
    if direction == 1 {
        *map.iter()
            .enumerate()
            .filter(|(_, c)| *c == &'.' || *c == &'#')
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
            .first()
            .unwrap()
    } else {
        let mut local_map = map.clone();
        local_map.reverse();
        local_map.len()
            - *local_map
                .iter()
                .enumerate()
                .filter(|(_, c)| *c == &'.' || *c == &'#')
                .map(|(i, _)| i)
                .collect::<Vec<usize>>()
                .first()
                .unwrap()
            - 1
    }
}

fn end_or_whitespace(map: &Vec<Vec<char>>, direction: (i32, i32), position: (i32, i32)) -> usize {
    if direction.1 == 1 {
        println!("{position:?}");
        *map.iter()
            .enumerate()
            .filter(|(_, c)| c[position.0 as usize] == '.' || c[position.0 as usize] == '#')
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
            .first()
            .unwrap()
    } else {
        let mut local_map = map.clone();
        local_map.reverse();
        map.len()
            - *map
                .iter()
                .enumerate()
                .filter(|(_, c)| c[position.0 as usize] == '.' || c[position.0 as usize] == '#')
                .map(|(i, _)| i)
                .collect::<Vec<usize>>()
                .first()
                .unwrap()
            - 1
    }
}

fn moving<F>(
    map: &Vec<Vec<char>>,
    fp: (i32, i32),
    limit: usize,
    mut f: F,
    drawing_map: &mut Vec<Vec<char>>,
) -> (i32, i32)
where
    F: FnMut(&Vec<Vec<char>>, i32, (i32, i32)) -> usize,
{
    let position: (i32, i32);
    if fp.1 >= limit as i32 {
        let new_line = f(map, 1, fp);
        if map[new_line][fp.0 as usize] == '#' {
            return (-2, -2);
        }
        drawing_map[new_line][fp.0 as usize] = 'V';
        position = (fp.0, new_line as i32);
    } else if map[fp.1 as usize][fp.0 as usize].is_ascii_whitespace() {
        let new_line = f(map, 1, fp);
        if map[new_line][fp.0 as usize] == '#' {
            return (-2, -2);
        }
        drawing_map[new_line][fp.0 as usize] = 'V';
        position = (fp.0, new_line as i32);
    } else {
        if map[fp.1 as usize][fp.0 as usize] == '#' {
            return (-2, -2);
        }
        drawing_map[fp.1 as usize][fp.0 as usize] = 'V';
        position = fp;
    }
    position
}
