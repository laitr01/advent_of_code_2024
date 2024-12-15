use regex::Regex;

#[path = "utils/file.rs"]
mod file;

pub fn solve_the_puzzle_3_0() {
    match file::read_file("inputs/day_3_0.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
            let mut sum = 0;
            for cap in regex.captures_iter(&contents) {
                //println!("{} ", &cap[0]);
                sum += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
            }
            println!("{}", sum);
        }
    }
}


pub fn solve_the_puzzle_3_1() {
    match crate::file::read_file("inputs/day_3_1.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let regex = Regex::new(r"mul\((\d+),(\d+)\)|\bdo\(\)|\bdon't\(\)").unwrap();
            let mut sum = 0;
            let mut mul = true;
            for cap in regex.captures_iter(&contents) {
                //println!("{}", &cap[0])
                match &cap[0]  {
                    "do()" => mul = true,
                    "don't()" => mul = false,
                    _ => {
                        if mul {
                            sum += cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
                        }
                    }
                }
            }
            println!("{}", sum);
        }
    }
}