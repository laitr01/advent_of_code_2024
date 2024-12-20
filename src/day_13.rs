use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[path = "utils/file.rs"]
mod file;
use std::str::FromStr;

pub fn solve_the_puzzle_13_0() {
    match file::read_file("inputs/day_13_0.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut claw_machines = Vec::new();
            let mut button_a = (0, 0);
            let mut button_b = (0, 0);
            let mut prize = (0, 0);

            for line in contents.lines() {
                if line.is_empty() {
                    claw_machines.push(ClawMachine {
                        button_a,
                        button_b,
                        prize,
                    });
                    button_a = (0, 0);
                    button_b = (0, 0);
                    prize = (0, 0);
                    continue;
                }

                let tokens = line.split_whitespace().collect::<Vec<&str>>();
                match tokens.as_slice() {
                    ["Button", "A:", x, y] => {
                        if let Some(value) = x.strip_prefix("X+").unwrap().strip_suffix(",") {
                            button_a.0 = value.parse().unwrap_or(0);
                        }
                        if let Some(value) = y.strip_prefix("Y+") {
                            button_a.1 = value.parse().unwrap_or(0);
                        }
                    }
                    ["Button", "B:", x, y] => {
                        if let Some(value) = x.strip_prefix("X+").unwrap().strip_suffix(",") {
                            button_b.0 = value.parse().unwrap_or(0);
                        }
                        if let Some(value) = y.strip_prefix("Y+") {
                            button_b.1 = value.parse().unwrap_or(0);
                        }
                    }
                    ["Prize:", x, y] => {
                        if let Some(value) = x.strip_prefix("X=").unwrap().strip_suffix(",") {
                            prize.0 = value.parse().unwrap_or(0);
                        }
                        if let Some(value) = y.strip_prefix("Y=") {
                            prize.1 = value.parse().unwrap_or(0);
                        }
                    }
                    _ => {}
                }
            }
            // Push the last machine
            claw_machines.push(ClawMachine {
                button_a,
                button_b,
                prize,
            });

            let total_cost = 0;

            let mut total_cost = 0;

            for machine in claw_machines {
                let (a_x, a_y) = machine.button_a;
                let (b_x, b_y) = machine.button_b;
                let (p_x, p_y) = machine.prize;
                let mut min_cost = 0;
                let mut found = false;
                for a_press in 0..100 {
                    for b_press in 0..100 {
                        if a_x * a_press + b_x * b_press == p_x && a_y * a_press + b_y * b_press == p_y {
                            let cost = a_press * 3 + b_press;
                            if !found || cost < min_cost {
                                min_cost = cost;
                                found = true;
                            }
                        }
                    }
                }

                if found {
                    total_cost += min_cost;
                }
            }

            println!("13.0: {:?}", total_cost);
        }
    }
}

pub fn solve_the_puzzle_13_1() {
    match file::read_file("inputs/day_13_1.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut claw_machines = Vec::new();
            let mut button_a = (0, 0);
            let mut button_b = (0, 0);
            let mut prize = (0, 0);

            for line in contents.lines() {
                if line.is_empty() {
                    claw_machines.push(ClawMachine {
                        button_a,
                        button_b,
                        prize,
                    });
                    button_a = (0, 0);
                    button_b = (0, 0);
                    prize = (0, 0);
                    continue;
                }

                let tokens = line.split_whitespace().collect::<Vec<&str>>();
                match tokens.as_slice() {
                    ["Button", "A:", x, y] => {
                        if let Some(value) = x.strip_prefix("X+").unwrap().strip_suffix(",") {
                            button_a.0 = value.parse().unwrap_or(0);
                        }
                        if let Some(value) = y.strip_prefix("Y+") {
                            button_a.1 = value.parse().unwrap_or(0);
                        }
                    }
                    ["Button", "B:", x, y] => {
                        if let Some(value) = x.strip_prefix("X+").unwrap().strip_suffix(",") {
                            button_b.0 = value.parse().unwrap_or(0);
                        }
                        if let Some(value) = y.strip_prefix("Y+") {
                            button_b.1 = value.parse().unwrap_or(0);
                        }
                    }
                    ["Prize:", x, y] => {
                        if let Some(value) = x.strip_prefix("X=").unwrap().strip_suffix(",") {
                            prize.0 = value.parse().unwrap_or(0);
                        }
                        if let Some(value) = y.strip_prefix("Y=") {
                            prize.1 = value.parse().unwrap_or(0);
                        }
                    }
                    _ => {}
                }
            }
            // Push the last machine
            claw_machines.push(ClawMachine {
                button_a,
                button_b,
                prize,
            });

            let mut total_cost:i64 = 0;

            for machine in claw_machines {
                let (a_x, a_y) = machine.button_a;
                let (b_x, b_y) = machine.button_b;
                let (mut p_x, mut p_y) = machine.prize;
                let det = a_x * b_y - a_y * b_x;
                p_x += 10000000000000;
                p_y += 10000000000000;
                if det == 0 {
                    continue;
                }
                let a_press: i64 = (p_x * b_y - p_y * b_x) / det;
                let b_press: i64 = (a_x * p_y - a_y * p_x) / det;

                if a_x * a_press + b_x * b_press == p_x && a_y * a_press + b_y * b_press == p_y && b_press >= 0 && a_press >= 0 {
                    total_cost += a_press * 3 + b_press;
                }
            }

            println!("13.1: {:?}", total_cost);
        }
    }
}
fn split(s: &str, delimiter: &str) -> Vec<String> {
    s.split(delimiter).map(|x| x.to_string()).collect()
}

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64), // (X, Y) change for Button A
    button_b: (i64, i64), // (X, Y) change for Button B
    prize: (i64, i64),    // (X, Y) position of the prize
}
