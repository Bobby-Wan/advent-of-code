#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};


fn part1() {
    let file = std::fs::File::open("input2").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut total_score = 0;

    for line in lines {
        if let Ok(validated_line) = line {
            println!("{}", validated_line);
            let mut moves = validated_line.split(" ");
            let opponent_move = moves.next();
            let my_move = moves.next();

            let score = match (opponent_move, my_move) {
                (Some(op), Some(me)) => score_move(op, me),
                _ => { println!("Could not extract moves"); return; }
            };

            total_score += score;
        }
    }
    
    println!("Total score is {}", total_score);
}

fn part2() {
    let file = std::fs::File::open("input2").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut total_score = 0;

    for line in lines {
        if let Ok(validated_line) = line {
            println!("{}", validated_line);
            let mut parts = validated_line.split(" ");
            let opponent_move = parts.next();
            let strategy = parts.next();

            match (opponent_move, strategy) {
                (Some(opponent), Some(strat)) => { 
                    let my_move = choose_move(opponent, strat);
                    total_score += score_move(opponent, my_move.as_str());
                }
                _ => { println!("Could not extract moves"); return; }
            };
        }
    }
    
    println!("Total score is {}", total_score);
}

fn main() {
    part2();
}

fn score_move(op: &str, me: &str) -> i32 {
    match (op, me) {
        ("A", "X") => 1+3,
        ("B", "Y") => 2+3,
        ("C", "Z") => 3+3,
        ("A", "Y") => 2+6,
        ("A", "Z") => 3+0,
        ("B", "X") => 1+0,
        ("B", "Z") => 3+6,
        ("C", "X") => 1+6,
        ("C", "Y") => 2+0,
        (_, _) => { println!("Unexpected moves to score. Returning -1"); -1 }
    }
}

fn choose_move(op: &str, strategy: &str) -> String {
    match (op, strategy) {
        ("A", "X") => String::from("Z"),
        ("B", "Y") => String::from("Y"),
        ("C", "Z") => String::from("X"),
        ("A", "Y") => String::from("X"),
        ("A", "Z") => String::from("Y"),
        ("B", "X") => String::from("X"),
        ("B", "Z") => String::from("Z"),
        ("C", "X") => String::from("Y"),
        ("C", "Y") => String::from("Z"),
        (_, _) => { println!("Unexpected strategy. Returning empty string"); String::from("") }
    }
}
