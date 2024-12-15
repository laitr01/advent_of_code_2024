use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[path = "utils/file.rs"]
mod file;

pub fn solve_the_puzzle_10_0() {
    match file::read_file("inputs/day_10_0.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let grid = parse_map(contents.as_str());
            let sum = calculate_scores(&grid);
            println!("10.0: {:?}", sum);
        }
    }
}

pub fn solve_the_puzzle_10_1() {
    match file::read_file("inputs/day_10_1.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let grid = parse_map(contents.as_str());
            let sum = calculate_sum_of_ratings(&grid);
            println!("10.1: {:?}", sum);
        }
    }
}

type Grid = Vec<Vec<u8>>;

fn parse_map(input_map: &str) -> Grid {
    input_map
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn find_trailheads(grid: &Grid) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &value) in row.iter().enumerate() {
            if value == 0 {
                trailheads.push((r, c));
            }
        }
    }
    trailheads
}

fn calculate_scores(grid: &Grid) -> usize {
    let trailheads = find_trailheads(grid);
    trailheads
        .iter()
        .map(|&trailhead| bfs(grid, trailhead).0.len())
        .sum()
}

fn calculate_sum_of_ratings(grid: &Grid) -> usize {
    let trailheads = find_trailheads(&grid);
    let mut total_rating = 0;

    for (r, c) in trailheads {
        let (_, count) = bfs(&grid, (r, c));
        total_rating += count;
    }

    total_rating
}

fn bfs(grid: &Grid, root: (usize, usize)) -> (HashSet<(usize, usize)>,usize) {
    let mut queue = VecDeque::new();
    let mut goals: HashSet<(usize, usize)> = HashSet::new();
    let mut count = 0;
    queue.push_back(root);

    while let Some((row, col)) = queue.pop_front() {
        if grid[row][col] == 9 {
            goals.insert((row, col));
            count += 1;
            continue;
        }

        for (dr, dc) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let nr = row as isize + dr;
            let nc = col as isize + dc;

            if nr >= 0 && nc >= 0 && nr < grid.len() as isize && nc < grid[0].len() as isize  {
                let nr = nr as usize;
                let nc = nc as usize;
                if grid[nr][nc] == grid[row][col] + 1 {
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    (goals,count)
}

