use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[path = "utils/file.rs"]
mod file;

pub fn solve_the_puzzle_11_0() {
    match file::read_file("inputs/day_11_0.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut grid = parse_map(contents.as_str());
            let mut count = 0;
            for i in 0..grid.len() {
                count += count_stones(grid[i], 25);
            }
            println!("11.0: {:?}", count);
        }
    }
}

pub fn solve_the_puzzle_11_1() {
    match file::read_file("inputs/day_11_1.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut grid = parse_map(contents.as_str());
            let mut count = simulate_stones(grid, 75);
            println!("11.1: {:?}", count);
        }
    }
}

type Stones = Vec<i64>;

fn parse_map(input_map: &str) -> Stones {
    input_map
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

fn count_stones(number: i64, blinks: usize) -> i32 { //too much cost for 75 time blinks need to find another solution
    if blinks == 0  {
        return 1;
    }
    if number == 0 {
        return count_stones(1, blinks - 1);
    }
    let num_in_str = number.to_string();
    if num_in_str.len() % 2 == 0 {
        let (first_num, second_num) = num_in_str.split_at(num_in_str.len() / 2);
        return count_stones( first_num.parse::<i64>().unwrap(), blinks - 1) +
            count_stones( second_num.parse::<i64>().unwrap() , blinks - 1);
    }
    count_stones( number * 2024, blinks - 1)
}

fn transform_stone_counts(stone_counts: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut new_counts = HashMap::new();

    for (stone, count) in stone_counts {
        if stone == 0 {
            *new_counts.entry(1).or_insert(0) += count; // Rule 1: 0 becomes 1
        } else if stone.to_string().len() % 2 == 0 {
            // Rule 2: Split the stone
            let stone_str = stone.to_string();
            let mid = stone_str.len() / 2;
            let left: i64 = stone_str[..mid].parse().unwrap();
            let right: i64 = stone_str[mid..].parse().unwrap();
            *new_counts.entry(left).or_insert(0) += count;
            *new_counts.entry(right).or_insert(0) += count;
        } else {
            // Rule 3: Multiply by 2024
            *new_counts.entry(stone * 2024).or_insert(0) += count;
        }
    }

    new_counts
}

fn simulate_stones(initial_stones: Vec<i64>, blinks: usize) -> i64 {
    // Initialize frequency map
    let mut stone_counts = HashMap::new();
    for stone in initial_stones {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }

    // Simulate transformations
    for _ in 0..blinks {
        stone_counts = transform_stone_counts(stone_counts);
    }

    // Count total number of stones
    stone_counts.values().sum()
}