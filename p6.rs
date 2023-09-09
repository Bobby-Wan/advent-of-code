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
    part1();
}

fn part1() {
    let file = File::open("input6").unwrap();
    let mut reader = io::BufReader::new(file);
    let mut input: String = "".to_string();
    reader.read_to_string(&mut input).unwrap();

    let mut chars = [' '; 14];
    let mut filled = 0;
    for (i, c) in input.chars().enumerate() {
        if filled < 14 {
            chars[filled] = c;
            filled += 1;
            
            if filled == 14 && all_different(&chars) {
                println!("{:?}", chars);
                println!("first marker on {}", i+1);
                return;
            }
        }
        else {
            slide(&mut chars, c);
            if all_different(&chars) {
                println!("{:?}", chars);
                println!("first marker on {}", i+1);
                return;
            }
        }
    }
}

fn slide(arr: &mut[char; 14], end: char) {
    for i in 0..13 {
        arr[i] = arr[i+1];
    }
    arr[13] = end;
}

fn all_different(arr: &[char; 14]) -> bool {
    for i in 0..14 {
        for j in (i+1)..14 {
            if arr[i] == arr[j] {
                return false;
            }
        }
    }

    return true;
}
