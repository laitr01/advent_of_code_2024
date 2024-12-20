use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[path = "utils/file.rs"]
mod file;
use std::str::FromStr;
use std::fs::File;
use std::io::{self, Write};

pub fn solve_the_puzzle_15_0() {
    match file::read_file("inputs/day_15_0.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            //println!("14.0: {:?}", q1 * q2 * q3 * q4);
        }
    }
}

pub fn solve_the_puzzle_15_1() {
    match file::read_file("inputs/day_15_1.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut robots: Vec<Robot> = contents.lines().map({
                |line|
                    line.parse::<Robot>().unwrap()
            }).collect();
        }
    }
}
fn split(s: &str, delimiter: &str) -> Vec<String> {
    s.split(delimiter).map(|x| x.to_string()).collect()
}

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
}

impl Robot {
    fn mov(&mut self, direction: char) {
        match direction {
            '>' => self.x += 1,
            '<' => self.x -= 1,
            'v' => self.y += 1,
            '^' => self.y -= 1,
            _ => panic!("Unknown direction: {}", direction)
        }
    }
}

#[derive(Debug)]
struct Obstacle {
    x: i32,
    y: i32,
}

impl Obstacle {
    fn mov(&mut self, direction: char) {
        match direction {
            '>' => self.x += 1,
            '<' => self.x -= 1,
            'v' => self.y += 1,
            '^' => self.y -= 1,
            _ => panic!("Unknown direction: {}", direction)
        }
    }
}

struct Warehouse {
    robots: Vec<Robot>,
    obstacles: Vec<Obstacle>,
    map: HashMap<(i32, i32), char>,
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
            x: x_pos,
            y: y_pos,
        })
    }
}


