use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[path = "utils/file.rs"]
mod file;

pub fn solve_the_puzzle_12_0() {
    match file::read_file("inputs/day_12_0.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let garden_map: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect::<Vec<char>>()).collect();
            let rows = garden_map.len();
            let cols = garden_map[0].len();
            let mut total_price = 0;


            let mut visited = vec![vec![false; cols]; rows];

            for i in 0..rows {
                for j in 0..cols {
                    if !visited[i][j] {
                        total_price += explore(&garden_map, &mut visited, (i, j), garden_map[i][j]).0;
                    }
                }
            }
            println!("12.0: {:?}", total_price);
        }
    }
}

pub fn solve_the_puzzle_12_1() {
    match file::read_file("inputs/day_12_1.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let garden_map: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect::<Vec<char>>()).collect();
            let rows = garden_map.len();
            let cols = garden_map[0].len();
            let mut total_price = 0;


            let mut visited = vec![vec![false; cols]; rows];

            for i in 0..rows {
                for j in 0..cols {
                    if !visited[i][j] {
                        total_price += explore(&garden_map, &mut visited, (i, j), garden_map[i][j]).1;
                    }
                }
            }
            println!("12.1: {:?}", total_price);
        }
    }
}

fn count_corners(region: &HashSet<(i32, i32)>) -> i32 {
    let mut vertices = HashSet::new();

    // Generate vertices by scaling by 2 for integer precision
    for &(x, y) in region {
        vertices.insert((2 * x - 1, 2 * y - 1));
        vertices.insert((2 * x + 1, 2 * y - 1));
        vertices.insert((2 * x + 1, 2 * y + 1));
        vertices.insert((2 * x - 1, 2 * y + 1));
    }

    let mut corners = 0;

    // Check each vertex
    for &(vx, vy) in &vertices {
        // Check the 4 adjacent points around the vertex
        let adjacent_points = [
            (vx - 1, vy - 1),
            (vx + 1, vy - 1),
            (vx + 1, vy + 1),
            (vx - 1, vy + 1),
        ];

        // Determine the configuration of the 4 adjacent points
        let mut config: i32 = 0;
        for &(ax, ay) in &adjacent_points {
            config <<= 1;
            // Check if the adjacent point is outside the region
            if !region.contains(&(ax / 2, ay / 2)) {
                config |= 1;
            }
        }

        // Count corners based on the configuration
        let outside_count = config.count_ones();
        if outside_count == 1 {
            corners += 1;
        } else if outside_count == 2 {
            // If diagonally opposite sides are outside, add 2
            if config == 0b1010 || config == 0b0101 {
                corners += 2;
            }
        } else if outside_count == 3 {
            corners += 1;
        }
    }

    corners
}


fn explore(garden_map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, (r, c): (usize, usize), plant: char) -> (i64, i64) {
    let mut area = 0;
    let mut perimeter = 0;
    let mut queue = VecDeque::new();
    let mut region: HashSet<(i32, i32)> = HashSet::new();
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    visited[r as usize][c as usize] = true;
    queue.push_back((r, c));
    region.insert((r as i32, c as i32));


    while let Some((nr, nc)) = queue.pop_front() {
        area += 1;
        for (dr, dc) in directions {
            let (n_row, n_col) = (nr as i32 + dr, nc as i32 + dc);
            if n_row < 0 || n_row >= garden_map.len() as i32 || n_col < 0 || n_col >= garden_map[0].len() as i32 {
                perimeter += 1;
                continue;
            }
            if garden_map[n_row as usize][n_col as usize] != plant {
                perimeter += 1;
                continue;
            }
            if !visited[n_row as usize][n_col as usize] {
                queue.push_back((n_row as usize, n_col as usize));
                visited[n_row as usize][n_col as usize] = true;
                region.insert((n_row as i32, n_col as i32));
            }
        }
    }
    let corners = count_corners(&region);
    (area * perimeter as i64, region.len() as i64 * corners as i64)
}