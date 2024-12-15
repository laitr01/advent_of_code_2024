use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[path = "utils/file.rs"]
mod file;

pub fn solve_the_puzzle_6_0() {
    match file::read_file("inputs/day_6_0.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut current_pos: (i32, i32) = (0, 0);
            let mut current = '-';
            let obstacle = '#';
            let mut map: Vec<Vec<char>> = vec![];

            contents.lines().enumerate().for_each(|(row, rv)| {
                let mut rchar: Vec<char> = vec![];
                rv.chars().enumerate().for_each(|(col, cv)| {
                    rchar.push(cv);
                    if !get_direction(&cv).is_none() {
                        current_pos = (row as i32, col as i32);
                        current = cv;
                    }
                });
                map.push(rchar);
            });
            let mut visited: HashSet<(i32, i32)> = HashSet::new();
            loop {
                let current_direction = get_direction(&current);
                let next_pos = (current_direction.unwrap().1 + current_pos.0, current_direction.unwrap().0 + current_pos.1);
                if next_pos.0 < 0 ||
                    next_pos.1 < 0 ||
                    next_pos.0 >= map[0].len() as i32||
                    next_pos.1 >= map.len() as i32 {
                    break;
                }
                let next = map[next_pos.0 as usize][next_pos.1 as usize];
                if next == obstacle {
                    current = get_new_direction(current);
                    continue;
                }
                visited.insert((next_pos.0, next_pos.1));
                current_pos = next_pos;
            }

            println!("6.0: {}", visited.len());
        }
    }
}

fn get_new_direction(current_direction: char) -> char {
    match current_direction {
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        '^' => '>',
        _ => {
            current_direction
        }
    }
}

pub fn solve_the_puzzle_6_1() {
    match crate::file::read_file("inputs/day_6_1.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let (map, start, start_dir) = parse_map(contents.as_str());
            let result = find_loop_positions(map, start, start_dir);
            println!("6.1: {}", result);
        }
    }
}

fn get_next_direction(direction: Direction) -> Direction {
    match direction {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("Invalid direction"),
    }
}
type Point = (usize, usize);
type Direction = char;

fn parse_map(input: &str) -> (Vec<Vec<char>>, Point, Direction) {
    let mut map = vec![];
    let mut start_position = (0, 0);
    let mut start_direction = '^';

    for (y, line) in input.lines().enumerate() {
        let row: Vec<char> = line.chars().collect();
        if let Some(x) = row.iter().position(|&c| c == '^' || c == 'v' || c == '<' || c == '>') {
            start_position = (x, y);
            start_direction = row[x];
        }
        map.push(row);
    }

    (map, start_position, start_direction)
}
fn move_position(pos: Point, direction: Direction) -> Option<Point> {
    match direction {
        '^' => Some((pos.0, pos.1.saturating_sub(1))),
        '>' => Some((pos.0 + 1, pos.1)),
        'v' => Some((pos.0, pos.1 + 1)),
        '<' => Some((pos.0.saturating_sub(1), pos.1)),
        _ => None,
    }
}

fn simulate_patrol(
    map: &Vec<Vec<char>>,
    start: Point,
    start_dir: Direction
) -> Option<HashSet<Point>> {
    use std::collections::HashSet;

    let mut visited = HashSet::new();
    let mut position = start;
    let mut direction = start_dir;

    // Map directions to coordinate deltas
    let directions: HashMap<char, (isize, isize)> = vec![
        ('^', (0, -1)),  // Up
        ('>', (1, 0)),   // Right
        ('v', (0, 1)),   // Down
        ('<', (-1, 0))   // Left
    ]
        .into_iter()
        .collect();

    loop {
        // Track current state (position and direction)
        if !visited.insert((position, direction)) {
            // If revisiting the same position with the same direction, it's a loop
            return Some(visited.iter().map(|&(pos, _)| pos).collect());
        }

        // Calculate the next position based on the current direction
        let (dx, dy) = directions[&direction];
        let next_x = position.0 as isize + dx;
        let next_y = position.1 as isize + dy;

        // Check if the guard exits the map
        if next_x < 0 || next_y < 0 || next_x >= map[0].len() as isize || next_y >= map.len() as isize {
            break; // Guard leaves the map
        }

        let next_pos = (next_x as usize, next_y as usize);

        // Check if there's an obstacle in the next position
        if map[next_pos.1][next_pos.0] == '#' {
            // Turn right (change direction)
            direction = match direction {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => panic!("Invalid direction"),
            };
        } else {
            // Move forward
            position = next_pos;
        }
    }

    None // No loop detected
}

fn find_loop_positions(map: Vec<Vec<char>>, start: Point, start_dir: Direction) -> usize {
    let mut possible_positions = HashSet::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            //println!("at {} - {}", y, x);
            if map[y][x] == '.' && (x, y) != start {
                // Add obstruction and check for loops
                let mut test_map = map.clone();
                test_map[y][x] = '#';

                if simulate_patrol(&test_map, start, start_dir).is_some() {
                    possible_positions.insert((x, y));
                }
            }
        }
    }

    possible_positions.len()
}

fn get_direction(direction: &char) -> Option<(i32, i32)> {
    let mut directions: HashMap<char, (i32, i32)> = [
        ('>', (1, 0)),
        ('<', (-1, 0)),
        ('v', (0, 1)),
        ('^', (0, -1))
    ].iter().cloned().collect();
    directions.get(direction).copied()
}