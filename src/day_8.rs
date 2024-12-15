use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[path = "utils/file.rs"]
mod file;

pub fn solve_the_puzzle_8_0() {
    match file::read_file("inputs/day_8_0.txt") {
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

                        let anti_node_1 = Point { x: p1.x - delta_row, y: p1.y - delta_col };
                        let anti_node_2 = Point { x: p2.x + delta_row, y: p2.y + delta_col };

                        if anti_node_1.x >= 0 && anti_node_1.y >= 0 && anti_node_1.x < max_rows && anti_node_1.y < max_cols {
                            antinodes.insert(anti_node_1);
                        }
                        if anti_node_2.x >= 0 && anti_node_2.y >= 0 && anti_node_2.x < max_rows && anti_node_2.y < max_cols {
                            antinodes.insert(anti_node_2);
                        }
                    }
                }
            }
            println!("8.0: {}", antinodes.len());
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

