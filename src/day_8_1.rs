use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[path = "utils/file.rs"]
mod file;

pub fn solve_the_puzzle_8_1() {
    match file::read_file("inputs/day_8_1.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut antennas = HashMap::new();
            let mut antinodes = HashSet::new();
            contents
                .lines()
                .enumerate()
                .flat_map(|(row, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(move |(col, ch)| if ch != '.' { Some((ch, row, col)) } else { None })
                })
                .for_each(|(ch, row, col)| {
                    antennas.entry(ch).or_insert_with(Vec::new).push(Point { x: row as i32, y: col as i32 });
                });

            let max_rows = contents.lines().count() as i32;
            let max_cols = contents
                .lines()
                .map(|line| line.chars().count())
                .max()
                .unwrap_or(0) as i32;

            for (_, positions) in antennas {
                // Check every pair of antennas
                for i in 0..positions.len() {
                    for j in i + 1..positions.len() {
                        let p1 = positions[i];
                        let p2 = positions[j];

                        let delta_row = p2.x - p1.x;
                        let delta_col = p2.y - p1.y;


                        let mut anti_node_row_1 = p1.x;
                        let mut anti_node_col_1 = p1.y;

                        while anti_node_row_1 >= 0 && anti_node_col_1 >= 0 && anti_node_row_1 < max_rows && anti_node_col_1 < max_cols {
                            antinodes.insert(Point { x: anti_node_row_1, y: anti_node_col_1 });
                            anti_node_col_1 -= delta_col;
                            anti_node_row_1 -= delta_row;
                        }
                        let mut anti_node_row_2 = p2.x;
                        let mut anti_node_col_2 = p2.y;

                        while anti_node_row_2 >= 0 && anti_node_col_2 >= 0 && anti_node_row_2 < max_rows && anti_node_col_2 < max_cols {
                            antinodes.insert(Point { x: anti_node_row_2, y: anti_node_col_2 });
                            anti_node_col_2 += delta_col;
                            anti_node_row_2 += delta_row;
                        }
                    }
                }
            }
            println!("8.1: {}", antinodes.len());
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

