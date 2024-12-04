// use std::fs::File;
// use std::io::prelude::*;
use std::io::{self, BufRead};

fn diff(a: i32, b: i32) -> i32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn part1(file: &str) {
    let file = std::fs::File::open(file).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut total_diff: i32 = 0;

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            let mut locations = line.split("   ");
            let l1 = locations.next();
            let l2 = locations.next();

            match (l1, l2) {
                (Some(l1), Some(l2)) => {
                    list1.push(l1.parse::<i32>().unwrap());
                    list2.push(l2.parse::<i32>().unwrap());
                }
                _ => {
                    println!("Could not extract moves");
                    return;
                }
            };
        }
    }

    list1.sort();
    list2.sort();

    for i in 0..list1.len() {
        let d: i32 = diff(*list1.get(i).unwrap(), *list2.get(i).unwrap());
        total_diff += d;
    }

    println!("total diff {}", total_diff)
}

fn part2(file: &str) {
    let file = std::fs::File::open(file).unwrap();
    let lines = io::BufReader::new(file).lines();

    // let mut total_diff: i32 = 0;

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            let mut locations = line.split("   ");
            let l1 = locations.next();
            let l2 = locations.next();

            match (l1, l2) {
                (Some(l1), Some(l2)) => {
                    list1.push(l1.parse::<i32>().unwrap());
                    list2.push(l2.parse::<i32>().unwrap());
                }
                _ => {
                    println!("Could not extract moves");
                    return;
                }
            };
        }
    }

//     list1.sort();
//     list2.sort();

    let mut similarity_score = 0;
    for l1 in &list1 {
        let mut count = 0;
        for l2 in &list2 {
            if l1 == l2 {
                count += 1;
            }
        }
        similarity_score += l1 * count;
    }

    println!("total similarity score {}", similarity_score)
}

fn main() {
    // part1("p1.1");
    part2("p1.in");
}
