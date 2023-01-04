#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};


fn main() {
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
