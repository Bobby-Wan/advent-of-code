#![feature(iter_collect_into)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::fs::read_to_string;
use std::io::Read;
use std::io::Seek;

use std::cell::RefCell;

fn main() {
    part1().unwrap();
}

fn part1() -> Result<(), std::io::Error> {
    let file = File::open("input5").unwrap();
    let reader = io::BufReader::new(file);
    
    // let mut lines = reader.lines();
    // let line_count = lines.count();
    let mut lines = reader.lines();

    // if line_count == 0 {
    //     return Err(std::io::Error::new(io::ErrorKind::Other, "zero lines read from input file"));
    // }

    let mut line = lines.next()
        .expect("Did not find first line.")
        .expect("Can not parse first line.");

    let columns = line.len() / 4 + 1;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; columns];

    //populate stacks
    loop {
        for (i, char) in line.chars().enumerate() {
            if char >= 'A' && char <= 'Z' {
                stacks[i/4].push(char);
            }
        }

        line = lines.next()
            .expect("No line found.")
            .expect("Could not parse line.");

        if line.is_empty() {
            break;
        }
    }

    //put in normal stack order, as elements were added top-down
    for stack in &mut stacks {
        stack.reverse();
    }

    for line in lines {
        let line = line.expect("could not parse line");
        let words:Vec<&str> = line.split(" ").collect();
        let times = words[1].parse::<usize>().expect("Parsing \'times\' failed");
        let from = words[3].parse::<usize>().expect("Parsing \'from\' failed");
        let to = words[5].parse::<usize>().expect("Parsing \'to\' failed");

        let mut local_stack = Vec::with_capacity(times);
        for i in 0..times {
            let to_move = stacks[from-1].pop()
                .expect("No element found when popping.");

            local_stack.push(to_move);
        }

        for i in 0..times {
            let to_push = local_stack.pop()
                .expect("No element in local stack.");
            stacks[to-1].push(to_push);
        }
    }

    for stack in &mut stacks {
        let top = stack.pop();
        match top {
            Some(top) => { print!("{top}"); },
            None => { print!(" "); }
        }
    }
    Ok(())
}


// fn part1() {
//     let mut file = File::open("input5").unwrap();
//     let buf_reader = io::BufReader::new(file.by_ref());
//     let lines = buf_reader.lines();

//     let stacks: Vec<Vec<char>> = Vec::new();


//     let mut column_line_number = 0;

//     for (num, line) in lines.enumerate() {
//         match line {
//             Err(err) => {
//                 panic!("{}", err);
//             },
//             Ok(line) => {
//                 if line.is_empty() {
//                     column_line_number = num - 1;
//                     break;
//                 }
//             }
//         }
//     }

//     file.seek(std::io::SeekFrom::Start(0)).expect("Unable to revert file to position 0 on first try");
//     let buf_reader = io::BufReader::new(file.by_ref());
//     let mut lines = buf_reader.lines();

//     let column_line = lines.nth(column_line_number)
//         .unwrap()
//         .expect("Invalid column line in input");
//     let num_of_columns = parse_number_of_columns(&column_line);

//     file.seek(std::io::SeekFrom::Start(0)).expect("Unable to revert file to position 0 on second try");
//     let buf_reader = io::BufReader::new(file.by_ref());
//     // let buf_reader = io::BufReader::new(file);
//     let mut lines = buf_reader.lines();

//     for (num, Ok(line)) in lines.enumerate() {
        
//     }
    

// }

fn parse_number_of_columns(s: &str) -> usize {
    s.split_whitespace().count()
}

// fn parse_line_to_stack_elements(s: &str, stack: Vec<Vec<char>>) -> Result<(), ()> {
//     s.
//     for char in s.chars() {
        
//     }


//     Err(())
// }

struct Type<T> {
    data: RefCell<Vec<T>>
}

// Example of runtime error on borrowing mutably twice!!!
fn refcell() {
    let t = Type::<i32>{ data: RefCell::new(vec![]) };
    let mut d1 = t.data.borrow_mut();
    let mut d2 = t.data.borrow_mut();
    d1.push(5);
    d2.push(5);
    d1.push(5);
    d2.push(5);
}
