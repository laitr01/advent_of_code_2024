use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[path = "utils/file.rs"]
mod file;

pub fn solve_the_puzzle_5_0() {
    match file::read_file("inputs/day_5_0.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut rule_map: HashMap<i64, Vec<i64>> = HashMap::new();
            let mut updates: Vec<Vec<i64>> = vec![];

            let cs: Vec<&str> = contents.split("break").collect();
            for rule_line in cs[0].split("\n") {
                let r: Vec<i64> = rule_line.split("|").filter_map(|r| r.trim().parse::<i64>().ok()).collect();
               if r.len() > 1 {
                   rule_map.entry(r[0]).or_insert_with(Vec::new).push(r[1]);
               }
            }
            for update in cs[1].split('\n') {
                let u = update.split(",").filter_map(|item| item.trim().parse::<i64>().ok() ).collect::<Vec<i64>>();
                if u.len() > 0 {
                    updates.push(u);
                }
            }

            let mut sum = 0;

            for u in updates {
                if is_update_valid(&u, &mut rule_map) {
                    sum += u[u.len()/2];
                }
            }
            println!("{}", sum);
        }
    }
}


fn is_update_valid(update: &Vec<i64>, rules: &mut HashMap<i64, Vec<i64>>) -> bool {
    let mut valid: bool = true;

    update.iter().enumerate().for_each(|(index, &page)| {
        rules.entry(page).or_insert_with(Vec::new).iter().enumerate().for_each(|(_,&v)| {
           if let Some(pos) = update.iter().position(|&p| p == v) {
               if (pos < index) {
                   valid = false;
               }
           }
        })
    });

    valid
}


pub fn solve_the_puzzle_5_1() {
    match crate::file::read_file("inputs/day_5_1.txt") {
        Err(err) => println!("Error: {:?}", err),
        Ok(contents) => {
            let mut rule_map: HashMap<i64, Vec<i64>> = HashMap::new();
            let mut updates: Vec<Vec<i64>> = vec![];

            let cs: Vec<&str> = contents.split("break").collect();
            for rule_line in cs[0].split("\n") {
                let r: Vec<i64> = rule_line.split("|").filter_map(|r| r.trim().parse::<i64>().ok()).collect();
                if r.len() > 1 {
                    rule_map.entry(r[0]).or_insert_with(Vec::new).push(r[1]);
                }
            }
            for update in cs[1].split('\n') {
                let u = update.split(",").filter_map(|item| item.trim().parse::<i64>().ok() ).collect::<Vec<i64>>();
                if u.len() > 0 {
                    updates.push(u);
                }
            }

            let mut sum = 0;

            for update in updates {
                if !is_update_valid(&update, &mut rule_map) {
                    let ordered_update = reorder(&update, &mut rule_map);
                    sum += ordered_update[ordered_update.len()/2];
                }
            }
            println!("{}", sum);
        }
    }
}

//use topological sort to reoder update followed by the rules
fn reorder(update: &Vec<i64>, rules: &mut HashMap<i64, Vec<i64>>) -> Vec<i64> {
    let mut visited: HashMap<i64,usize> = HashMap::new();
    let mut adjacent: HashMap<i64, Vec<i64>> = HashMap::new();
    update.iter().enumerate().for_each(|(index, &page)| {
        adjacent.entry(page).or_insert_with(Vec::new);
        visited.entry(page).or_insert(0);
    });

    rules.iter().for_each(|(&before,after_list)| {
        if update.contains(&before) {
            after_list.iter().for_each(|&after| {
                if update.contains(&after) {
                    adjacent.entry(before).or_default().push(after);
                    *visited.entry(after).or_insert(0) +=1;
                }
            })
        }
    });

    let mut queue: VecDeque<i64> = VecDeque::new();
    visited.iter().filter(|(&key, &count)| count == 0 ).for_each(|(&key, &count)| {
        queue.push_back(key);
    });

    let mut sorted :Vec<i64> = vec![];

    while let Some(page) = queue.pop_front() {
        sorted.push(page);
        if let Some(neighbors) = adjacent.get(&page) {
            for &neighbor in neighbors {
                if let Some(pos) = visited.get_mut(&neighbor) {
                    *pos -= 1;
                    if *pos == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    sorted
}