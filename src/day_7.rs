use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[path = "utils/file.rs"]
mod file;

pub fn solve_the_puzzle_7_0() {
    match file::read_file("inputs/day_7_0.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let parsed_data = parse_input(contents.as_str());
            let result = total_calibration_result(&parsed_data);
            println!("7.0: {}", result);
        }
    }
}




fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines() // Split the input into lines
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect(); // Split each line at the colon
            let test_value = parts[0].trim().parse::<i64>().unwrap(); // Parse the test value
            let numbers = parts[1]
                .split_whitespace() // Split numbers by whitespace
                .map(|n| n.parse::<i64>().unwrap()) // Parse each number to i32
                .collect::<Vec<i64>>(); // Collect into a Vec<i32>
            (test_value, numbers) // Return as a tuple
        })
        .collect() // Collect all tuples into a Vec
}

fn evaluate_expression(numbers: &[i64], operators: &[char]) -> i64 {
    let mut result = numbers[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            _ => panic!("Unsupported operator"),
        }
    }
    result
}

fn is_valid_equation(test_value: i64, numbers: &[i64]) -> bool {
    let n = numbers.len();
    let operator_positions = n - 1;
    let operator_combinations = 1 << operator_positions; // 2^(n-1) combinations

    for i in 0..operator_combinations {
        let mut operators = Vec::new();
        for j in 0..operator_positions {
            if (i >> j) & 1 == 1 {
                operators.push('*');
            } else {
                operators.push('+');
            }
        }
        if evaluate_expression(numbers, &operators) == test_value {
            return true;
        }
    }
    false
}

fn total_calibration_result(equations: &[(i64, Vec<i64>)]) -> i64 {
    let mut total = 0;
    for &(test_value, ref numbers) in equations {
        if is_valid_equation(test_value, numbers) {
            total += test_value;
        }
    }
    total
}