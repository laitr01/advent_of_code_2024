use std::collections::{HashMap};

#[path = "utils/file.rs"]
mod file;

pub fn solve_the_puzzle_1_0() {
    match file::read_file("inputs/day_1_0.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut first_cols: Vec<i64> = Vec::new();
            let mut second_cols: Vec<i64> = Vec::new();
            for line in contents.lines() {
                let parts = line.split("   ").collect::<Vec<&str>>();
                first_cols.push(parts[0].parse::<i64>().unwrap());
                second_cols.push(parts[1].parse::<i64>().unwrap());
            }
            first_cols.sort();
            second_cols.sort();

            let mut sum = 0;
            for i in 0..first_cols.len() {
                sum += (first_cols[i] - second_cols[i]).abs();
            }

            println!("{}", sum);
        }
    }
}


pub fn solve_the_puzzle_1_1() {
    match crate::file::read_file("inputs/day_1_1.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut first_col_map : HashMap<i64, i64> = HashMap::new();
            let mut second_col_map: HashMap<i64, i64> = HashMap::new();
            for line in contents.lines() {
                let parts = line.split("   ").collect::<Vec<&str>>();
                let part_1 = parts[0].parse::<i64>().unwrap();
                let part_2 = parts[1].parse::<i64>().unwrap();
                *first_col_map.entry(part_1).or_insert(0)+=1;
                *second_col_map.entry(part_2).or_insert(0)+=1;
            }
            let mut sum = 0;

            for num in first_col_map.keys() {
                if second_col_map.contains_key(num) {
                    let count_1 = second_col_map.get(num).unwrap();
                    let count_2 = first_col_map.get(num).unwrap();
                    sum += num * count_1 * count_2;
                }
            }

            println!("{}", sum);
        }
    }
}