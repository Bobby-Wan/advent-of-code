use std::io::{self, BufRead};

fn levels_are_safe(levels: &Vec<i32>) -> bool {
    if levels_are_safe_inner(levels) {
        return true;
    }

    for i in 0..levels.len() {
        if levels_are_safe_without(levels, i) {
            return true;
        }
    }

    return false
}

fn levels_are_safe_inner(levels: &Vec<i32>) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let mut increasing = true;
    if levels.get(0).unwrap() > levels.get(1).unwrap() {
        increasing = false;
    }

    let mut current_level = levels.get(0).unwrap();
    for i in 1..levels.len() {
        let next_level = levels.get(i).unwrap();
        let diff = next_level - current_level;
        if increasing {
            if diff > 3 || diff < 1 {
                return false;
            }
        } else {
            if diff < -3 || diff > -1 {
                return false;
            }
        }
        current_level = next_level;
    }

    return true
}

fn levels_are_safe_without(levels: &Vec<i32>, index: usize) -> bool {
    let mut levels = levels.clone();
    levels.remove(index);

    if levels.len() < 2 {
        return true;
    }

    let mut increasing = true;
    if levels.get(0).unwrap() > levels.get(1).unwrap() {
        increasing = false;
    }

    let mut current_level = levels.get(0).unwrap();
    for i in 1..levels.len() {
        let next_level = levels.get(i).unwrap();
        let diff = next_level - current_level;
        if increasing {
            if diff > 3 || diff < 1 {
                return false;
            }
        } else {
            if diff < -3 || diff > -1 {
                return false;
            }
        }
        current_level = next_level;
    }

    return true
}
fn part1() {
    let file = std::fs::File::open("p2.in").unwrap();
    let lines = io::BufReader::new(file).lines();

    // let mut reports = Vec::new();
    
    let mut safe_levels = 0;
    for line in lines {
        if let Ok(line) = line {
            let mut levels = Vec::new();
            let elems = line.split(" ");
            for e in elems {
                levels.push(e.parse::<i32>().unwrap())
            }

            if levels_are_safe_inner(&levels) {
                safe_levels += 1;
            }
        }
    }

    println!("safe levels: {}", safe_levels);
}

fn part2() {
    let file = std::fs::File::open("p2.in").unwrap();
    let lines = io::BufReader::new(file).lines();

    // let mut reports = Vec::new();
    
    let mut safe_levels = 0;
    for line in lines {
        if let Ok(line) = line {
            let mut levels = Vec::new();
            let elems = line.split(" ");
            for e in elems {
                levels.push(e.parse::<i32>().unwrap())
            }

            if levels_are_safe(&levels) {
                safe_levels += 1;
            }
        }
    }

    println!("safe levels: {}", safe_levels);
}

fn main() {
    part2();
}
