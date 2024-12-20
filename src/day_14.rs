use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[path = "utils/file.rs"]
mod file;
use std::str::FromStr;
use std::fs::File;
use std::io::{self, Write};

pub fn solve_the_puzzle_14_0() {
    match file::read_file("inputs/day_14_0.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut robots: Vec<Robot> = contents.lines().map({
                |line|
                    line.parse::<Robot>().unwrap()
            }).collect();
            let (row, col) = (103, 101);
            let (row_mid, col_mid) = (row / 2, col / 2);

            robots.iter_mut().for_each(|robot| {
                robot.move_with_wrap(col, row, 100);
            });

            //let ro: Vec<&Robot> = robots.iter().filter(|robot| robot.x_pos != row_mid && robot.y_pos != col_mid).collect();

            let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
            for robot in &mut robots {
                if robot.x_pos < row_mid && robot.y_pos < col_mid {
                    q1 += 1;
                }
                if robot.x_pos < row_mid && robot.y_pos > col_mid {
                    q3 += 1;
                }
                if robot.x_pos > row_mid && robot.y_pos > col_mid {
                    q4 += 1;
                }
                if robot.x_pos > row_mid && robot.y_pos < col_mid {
                    q2 += 1;
                }
            }
            println!("14.0: {:?}", q1 * q2 * q3 * q4);
        }
    }
}

pub fn solve_the_puzzle_14_1() -> io::Result<()> {
    match file::read_file("inputs/day_14_1.txt") {
        Err(err) => Ok(()),
        Ok(contents) => {
            let mut robots: Vec<Robot> = contents.lines().map({
                |line|
                    line.parse::<Robot>().unwrap()
            }).collect();

            let (row, col) = (103, 101);
            let mut str_builder = String::new();
            for i in 0..10000 {
                let mut grid: Vec<Vec<bool>> = vec![vec![false; col]; row];
                robots.iter_mut().for_each(|robot| {
                    robot.move_with_wrap(col as i32, row as i32, 1);
                    grid[robot.x_pos as usize][robot.y_pos as usize] = true;
                });

                str_builder.push_str(&format!("Step {}", i+1));
                str_builder.push('\n');
                for i in 0..row {
                    for j in 0..col {
                        if grid[i][j] {
                            str_builder.push('#');
                        } else {
                            str_builder.push('.');
                        }
                    }
                    str_builder.push('\n');
                }
            }
            let mut file = File::create("output.txt")?;
            file.write_all(str_builder.as_bytes())?; // Convert string to bytes
            Ok(())
        }
    }
}
fn split(s: &str, delimiter: &str) -> Vec<String> {
    s.split(delimiter).map(|x| x.to_string()).collect()
}

#[derive(Debug)]
struct Robot {
    x_pos: i32,
    y_pos: i32,
    velocity_x: i32,
    velocity_y: i32,
}

impl Robot {
    fn move_with_wrap(&mut self, col: i32, row: i32, steps: i32) {
        self.x_pos = (self.x_pos + self.velocity_x * steps).rem_euclid(row);
        self.y_pos = (self.y_pos + self.velocity_y * steps).rem_euclid(col);
    }
}
impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Invalid input format".to_string());
        }

        let pos_part = parts[0].trim_start_matches("p=").split(',').collect::<Vec<&str>>();
        let vel_part = parts[1].trim_start_matches("v=").split(',').collect::<Vec<&str>>();

        if pos_part.len() != 2 || vel_part.len() != 2 {
            return Err("Invalid coordinates format".to_string());
        }

        let y_pos = pos_part[0].parse::<i32>().map_err(|_| "Invalid x_pos".to_string())?;
        let x_pos = pos_part[1].parse::<i32>().map_err(|_| "Invalid y_pos".to_string())?;
        let velocity_y = vel_part[0].parse::<i32>().map_err(|_| "Invalid velocity_x".to_string())?;
        let velocity_x = vel_part[1].parse::<i32>().map_err(|_| "Invalid velocity_y".to_string())?;

        Ok(Robot {
            x_pos,
            y_pos,
            velocity_x,
            velocity_y,
        })
    }
}


