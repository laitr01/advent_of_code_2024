use regex::Regex;

#[path = "utils/file.rs"]
mod file;

pub fn solve_the_puzzle_4_0() {
    match file::read_file("inputs/day_4_0.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let directions:[(isize,isize);8] = [
                (0,1), //move to right 1
                (0,-1), //move to left 1
                (1,0), //move up 1
                (-1,0), //move down 1
                (1,1), //move up right
                (1,-1), //move up left
                (-1,-1), //move down left
                (-1,1), //move down right
            ];
            let word = "XMAS";
            let word_len = word.len();
            let word_chars: Vec<char> = word.chars().collect();
            let mut grid: Vec<Vec<char>> = vec![];
            for line in contents.lines() {
                grid.push(line.chars().collect());
            }
            let rows = grid.len();
            let cols = grid[0].len();
            let mut count = 0;

            for row in 0..rows {
                for col in 0..cols {
                    for &(dr,dc) in &directions {
                        let mut matched = true;
                        for i in 0..word_len {
                            let nr = row as isize + i as isize * dr;
                            let nc = col as isize + i as isize * dc;

                            if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize || grid[nr as usize][nc as usize] != word_chars[i]{
                                matched = false;
                                break;
                            }
                        }
                        if matched {
                            count += 1;
                        }
                    }
                }
            }
            println!("{}", count);
        }
    }
}


pub fn solve_the_puzzle_4_1() {
    match crate::file::read_file("inputs/day_4_1.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut grid: Vec<Vec<char>> = vec![];
            for line in contents.lines() {
                grid.push(line.chars().collect());
            }
            let rows = grid.len();
            let cols = grid[0].len();
            let mut count = 0;

            for r in 1..rows - 1 {
                for c in 1..cols - 1 {
                    if grid[r][c] == 'A' {
                        // Check diagonal directions
                        if grid[r - 1][c - 1] == 'M' &&
                            grid[r + 1][c + 1] == 'S' &&
                            grid[r - 1][c + 1] == 'M' &&
                            grid[r + 1][c - 1] == 'S' {
                            count += 1;
                        }
                        if grid[r - 1][c - 1] == 'S' &&
                            grid[r + 1][c + 1] == 'M' &&
                            grid[r - 1][c + 1] == 'M' &&
                            grid[r + 1][c - 1] == 'S' {
                            count += 1;
                        }
                        if grid[r - 1][c - 1] == 'M' &&
                            grid[r + 1][c + 1] == 'S' &&
                            grid[r - 1][c + 1] == 'S' &&
                            grid[r + 1][c - 1] == 'M' {
                            count += 1;
                        }
                        if grid[r - 1][c - 1] == 'S' &&
                            grid[r + 1][c + 1] == 'M' &&
                            grid[r - 1][c + 1] == 'S' &&
                            grid[r + 1][c - 1] == 'M' {
                            count += 1;
                        }
                    }
                }
            }
            println!("{}", count);
        }
    }
}