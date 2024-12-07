use std::io::{self, BufRead};

fn parse_mul(s: &str) -> usize {
    let mut result = 0;
    let mut split = s.split("mul");
    // skip first split before `mut`
    split.next();
    while let Some(s) = split.next() {
        let mut chars = s.chars();

        if s.len() < 5 {
            println!("s.len < 5");
            continue;
        }

        if chars.next().unwrap() != '(' {
            // println!("first char not (");
            continue;
        }

        let mut last: Option<char> = None;
        let n1s = chars
            .by_ref()
            .take_while(|c| {
                last = Some(*c);
                c.is_digit(10)
            })
            .collect::<String>();
        let n1 = n1s.parse::<usize>();
        if let Err(_) = n1 {
            println!("could not parse n1");
            continue;
        }
        let n1_len = n1s.len();

        let n1 = n1.unwrap();

        if n1_len + 4 > s.len() {
            println!("n1_len+4 > s.len()");
            continue;
        }

        if let None = last {
            println!("nothing found after n1 not found");
            continue;
        }

        let comma_char = last.unwrap();
        if comma_char != ',' {
            // println!("found {} instead of ,", comma_char);
            continue;
        }

        let n2s = chars
            .by_ref()
            .take_while(|c| {
                last = Some(*c);
                c.is_digit(10)
            })
            .collect::<String>();
        let n2 = n2s.parse::<usize>();
        if let Err(_) = n2 {
            println!("could not parse n2");
            continue;
        }
        let n2 = n2.unwrap();

        if let None = last {
            println!("nothing found after n2 not found");
            continue;
        }

        if n1_len + n2s.len() + 3 > s.len() {
            println!("n1 + n2 + 3 > s.len()");
            continue;
        }

        if Some(')') != last {
            println!("didn't find )");
            continue;
        }

        println!("{n1}x{n2}");
        result += n1 * n2;
    }

    return result;
}

fn part1() {
    let file = std::fs::File::open("p3.in").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut sum: usize = 0;

    for line in lines {
        if let Ok(line) = line {
            sum += parse_mul(&line);
        }
    }

    println!("total sum: {}", sum);
}

fn part2() {
    // let file = std::fs::File::open("p3.in").unwrap();
    // let input = io::BufReader::new(file);
    let input = std::fs::read_to_string("p3.in").unwrap();

    let mut sum: usize = 0;

    // for line in lines {
    //     if let Ok(line) = line {
    // println!("line:\n {line}");
    let mut split = input.split("do()");
    while let Some(s) = split.next() {
        let mut s = s.split("don't()");
        let before = s.next();
        match before {
            None => {
                continue;
            }
            Some("") => {
                continue;
            }
            Some(s) => {
                println!("using:\n{s}");
                let result = parse_mul(s);
                println!("{result}");
                sum += result;
            }
        }
    }
    // }
    // }

    println!("total sum: {}", sum);
}

fn main() {
    part2();
}
