use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[path = "utils/file.rs"]
mod file;


pub fn solve_the_puzzle_7_1() {
    match file::read_file("inputs/day_7_1.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let total_calibration_result = calculate_total_calibration_result(contents.as_str());
            println!("7.1: {}", total_calibration_result);
        }
    }
}

fn parse_input(input: &str) -> Vec<(i64, Vec<String>)> {
    input
        .lines() // Split the input into lines
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect(); // Split each line at the colon
            let test_value = parts[0].trim().parse::<i64>().unwrap(); // Parse the test value
            let numbers = parts[1]
                .split_whitespace() // Split numbers by whitespace
                .map(|n| n.to_string()) // Collect into Vec<String> (as strings)
                .collect::<Vec<String>>(); // Collect into a Vec<String>
            (test_value, numbers) // Return as a tuple
        })
        .collect() // Collect all tuples into a Vec
}
// Helper function to apply concatenation || between two numbers
fn concat(left: &str, right: &str) -> String {
    format!("{}{}", left, right)
}

// Helper function to evaluate an expression left-to-right
fn eval_expression(numbers: &Vec<String>, operators: &Vec<String>) -> i64 {
    let mut result = numbers[0].parse::<i64>().unwrap();
    for i in 0..operators.len() {
        let next_num = numbers[i + 1].parse::<i64>().unwrap();
        match operators[i].as_str() {
            "+" => result += next_num,
            "*" => result *= next_num,
            "||" => result = concat(&result.to_string(), &next_num.to_string()).parse().unwrap(),
            _ => panic!("Unknown operator!"),
        }
    }
    result
}

// Function to generate all possible operator combinations for the expression
fn generate_operator_combinations(operators: Vec<&str>, n: usize) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let total_ops = n - 1;
    let mut ops = vec!["+"; total_ops];

    loop {
        result.push(ops.iter().map(|&s| s.to_string()).collect::<Vec<String>>());
        let mut i = total_ops - 1;
        while i >= 0 {
            if ops[i] != operators[operators.len() - 1] {
                ops[i] = operators[operators.iter().position(|&x| x == ops[i]).unwrap() + 1];
                break;
            } else {
                ops[i] = operators[0];
            }
            if i == 0 {
                return result;
            }
            i -= 1;
        }
    }
}

fn calculate_total_calibration_result(input: &str) -> i64 {
    let parsed_data = parse_input(input);
    let operators = vec!["+", "*", "||"];
    let mut total = 0;

    for (test_value, numbers) in parsed_data {
        let n = numbers.len();
        let operator_combinations = generate_operator_combinations(operators.to_vec(), n);

        // Check each combination of operators
        for op_combination in operator_combinations {
            if eval_expression(&numbers, &op_combination) == test_value {
                total += test_value;
                break;
            }
        }
    }

    total
}