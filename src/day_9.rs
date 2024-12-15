use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[path = "utils/file.rs"]
mod file;

pub fn solve_the_puzzle_9_0() {
    match file::read_file("inputs/day_9_0.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut disk_map: Vec<String> = vec![];
            let mut block_id = 0;
            contents.lines().for_each(|line| {
                line.chars().enumerate().for_each(|(i, c)| {
                    for _ in 0..c.to_digit(10).unwrap() {
                        if (i + 1) % 2 == 0 {
                            disk_map.push(".".to_string());
                        } else {
                            let t = block_id.to_string();
                            disk_map.push(t);
                        }
                    }
                    if (i + 1) % 2 != 0 {
                        block_id += 1;
                    }
                });
            });
            let mut start: usize = 0;
            let mut end: usize = disk_map.len() - 1;
            while start <= end {
                if disk_map[start] == "." && disk_map[end] != "." {
                    disk_map[start] = disk_map[end].to_string();
                    disk_map[end] = ".".to_string();
                    start += 1;
                    end -= 1;
                } else if disk_map[start] == "." && disk_map[end] == "." {
                    end -= 1;
                } else if disk_map[start] != "." && disk_map[end] != "." {
                    start += 1;
                } else if disk_map[start] != "." && disk_map[end] == "." {
                    start += 1;
                    end -= 1;
                }
            }

            let mut sum: i64 = 0;
            for i in 0..disk_map.len() {
                if disk_map[i] == "." {
                    continue;
                }
                sum += disk_map[i].parse::<i64>().unwrap() * i as i64;
            }
            println!("9.0: {:?}", sum);
        }
    }
}

pub fn solve_the_puzzle_9_1() {
    match file::read_file("inputs/day_9_1.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut disk_map: Vec<String> = vec![];
            let mut file_id = 0;
            let mut files: Vec<(i32, i32, i32)> = vec![];
            contents.lines().for_each(|line| {
                line.chars().enumerate().for_each(|(i, c)| {
                    for _ in 0..c.to_digit(10).unwrap() {
                        if (i + 1) % 2 == 0 {
                            disk_map.push(".".to_string());
                        } else {
                            let t = file_id.to_string();
                            disk_map.push(t);
                        }
                    }
                    if (i + 1) % 2 != 0 {
                        let block_len = c.to_digit(10).unwrap() as i32;
                        files.push((file_id, block_len, disk_map.len() as i32 - block_len));
                        file_id += 1;
                    }
                });
            });
            let mut end: usize = files.len() - 1;
            let mut start: usize = 0;

            for i in (start..=end).rev().step_by(1) {
                let file = files.get(i).unwrap();
                let file_len = file.1;
                let file_index = file.2;
                let mut start_trace = 0;
                let end_trace = file_index;

                while start_trace < end_trace {
                    let mut len_valid = true;
                    for k in start_trace..(start_trace + file_len) {
                        if disk_map[k as usize] != "." {
                            len_valid = false;
                        }
                    }

                    if len_valid {
                        (0..file_len).for_each(|l| {
                            disk_map[(start_trace + l) as usize] = disk_map[(file_index + l) as usize].to_string();
                            disk_map[(file_index + l) as usize] = ".".to_string();
                        });
                        start_trace += file_len;
                        break;
                    } else {
                        start_trace += 1;
                    }
                }
            }


            let mut sum: i64 = 0;
            for i in 0..disk_map.len() {
                if disk_map[i] == "." {
                    continue;
                }
                sum += disk_map[i].parse::<i64>().unwrap() * i as i64;
            }
            println!("9.1: {:?}", sum);
        }
    }
}

