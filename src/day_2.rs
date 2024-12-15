
#[path = "utils/file.rs"]
mod file;

pub fn solve_the_puzzle_2_0() {
    match file::read_file("inputs/day_2_0.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut count_safe = 0;
            for line in contents.lines() {
                let parts: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|part| part.parse::<i32>().ok())
                    .collect();
                if is_safe(&parts) {
                    count_safe += 1;
                }
            }
            println!("{}", count_safe);
        }
    }
}


pub fn solve_the_puzzle_2_1() {
    match crate::file::read_file("inputs/day_2_1.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut count_safe = 0;
            for line in contents.lines() {
                let parts: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|part| part.parse::<i32>().ok())
                    .collect();
                if is_safe(&parts) {
                    count_safe += 1;
                } else {
                    if(0..parts.len()).any(|i| {
                        let mut parts = parts.clone();
                        parts.remove(i);
                        is_safe(&parts)
                    }){
                        count_safe += 1;
                    }
                }
            }
            println!("{}", count_safe);
        }
    }

}

fn is_safe(list: &Vec<i32>) -> bool {
    list.windows(2).all(|w| (1..=3).contains(&(w[1]-w[0]).abs()))
    && (list.windows(2).all(|w| w[0] > w[1]) || (list.windows(2).all(|w| w[0] < w[1])))
}