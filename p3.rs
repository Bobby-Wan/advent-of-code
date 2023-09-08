#![feature(iter_collect_into)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;

fn main() {
    part2();
}

// fn part1() {
//     let file = File::open("input3").unwrap();
//     let lines = io::BufReader::new(file).lines();

//     let mut total_score = 0;

//     for line in lines {
//         if let Ok(l) = line {
//             println!("{}", l);
//             let total_items_count = l.len();
//             println!("{}", total_items_count);
//             let (items_in_slot_1, items_in_slot_2) = l.split_at(total_items_count/2);
//             let opt_item = find_duplicate_item(items_in_slot_1, items_in_slot_2);
//             if let Some(item) = opt_item {
//                 let score = score_item(item);
//                 total_score += score;
//             }
//         }
//     }

//     println!("Total score: {}", total_score);
// }

fn part2() {
    let file = File::open("input3").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut total_score = 0;

    let mut elf_count = 0;
    let mut items: Vec<String> = vec!["".to_string(), "".to_string(), "".to_string()];
     
    for line in lines {
        if let Ok(l) = line {
            items[elf_count] = l;

            elf_count += 1;
            if elf_count == 3 {
                elf_count = 0;
                let matching_item = find_matching_item(&items).unwrap();
                total_score += score_item(matching_item);
            }
        }
    }

    println!("Total score: {}", total_score);
}

fn find_matching_item(items: &Vec<String>) -> Option<char> {
    let chars1: HashSet<char> = HashSet::from_iter(items[0].chars());
    let chars2: HashSet<char> = HashSet::from_iter(items[1].chars());
    let chars3: HashSet<char> = HashSet::from_iter(items[2].chars());
    
    let mut inter1: HashSet<char> = HashSet::new();
    let mut inter2: HashSet<char> = HashSet::new();

    let intersection1 = chars1.intersection(&chars2).collect_into(&mut inter1);
    let common_chars = intersection1.intersection(&chars3).collect_into(& mut inter2);

    if common_chars.len() != 1 {
        println!("Number of common characters is {}", common_chars.len());
        return None
    }

    Some(*common_chars.iter().next().unwrap())
}

fn is_uppercase(c: char) -> bool {
    c as u32 >= 65 && c as u32 <= 90
}

fn is_lowercase(c: char) -> bool {
    c as u32 >= 97 && c as u32 <= 122
}

fn score_item(item: char) -> u32 {
    if is_lowercase(item) { return item as u32 - 96; }
    if is_uppercase(item) { return item as u32 - 38; }

    0
}

fn find_duplicate_item(slot1: &str, slot2: &str) -> Option<char> {
    let mut chars1: Vec<char> = slot1.chars().collect();
    let mut chars2: Vec<char> = slot2.chars().collect();

    chars1.sort_by(|a,b| a.cmp(b));
    chars2.sort_by(|a,b| a.cmp(b));

    let mut i1 = 0;
    let mut i2 = 0;

    while i1 < chars1.len() {
        if chars1[i1] > chars2[i2] {
            if i2 < chars2.len() - 1 {
                i2 += 1;
            } else {
                return None;
            }
        } else if chars1[i1] < chars2[i2]{
            if i1 < chars1.len() - 1 {
                i1 += 1;
            } else {
                return None;
            }
        } else {
            return Some(chars1[i1]);
        }
    }

    None
}
