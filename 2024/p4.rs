use std::io::{self, BufRead};

fn check_down(m: &Vec<Vec<char>>, x: usize, y: usize) -> Option<bool> {
    let char1 = m.get(y)?.get(x)?;
    let char2 = m.get(y + 1)?.get(x)?;
    let char3 = m.get(y + 2)?.get(x)?;
    let char4 = m.get(y + 3)?.get(x)?;
    match (char1, char2, char3, char4) {
        ('X', 'M', 'A', 'S') => Some(true),
        _ => Some(false),
    }
}

fn check_up(m: &Vec<Vec<char>>, x: usize, y: usize) -> Option<bool> {
    if y < 3 {
        return None;
    }

    let char1 = m.get(y)?.get(x)?;
    let char2 = m.get(y - 1)?.get(x)?;
    let char3 = m.get(y - 2)?.get(x)?;
    let char4 = m.get(y - 3)?.get(x)?;
    match (char1, char2, char3, char4) {
        ('X', 'M', 'A', 'S') => Some(true),
        _ => Some(false),
    }
}

fn check_right(m: &Vec<Vec<char>>, x: usize, y: usize) -> Option<bool> {
    let char1 = m.get(y)?.get(x)?;
    let char2 = m.get(y)?.get(x + 1)?;
    let char3 = m.get(y)?.get(x + 2)?;
    let char4 = m.get(y)?.get(x + 3)?;
    match (char1, char2, char3, char4) {
        ('X', 'M', 'A', 'S') => Some(true),
        _ => Some(false),
    }
}

fn check_left(m: &Vec<Vec<char>>, x: usize, y: usize) -> Option<bool> {
    if x < 3 {
        return None;
    }

    let char1 = m.get(y)?.get(x)?;
    let char2 = m.get(y)?.get(x - 1)?;
    let char3 = m.get(y)?.get(x - 2)?;
    let char4 = m.get(y)?.get(x - 3)?;
    match (char1, char2, char3, char4) {
        ('X', 'M', 'A', 'S') => Some(true),
        _ => Some(false),
    }
}

fn check_diag_down_right(m: &Vec<Vec<char>>, x: usize, y: usize) -> Option<bool> {
    let char1 = m.get(y)?.get(x)?;
    let char2 = m.get(y + 1)?.get(x + 1)?;
    let char3 = m.get(y + 2)?.get(x + 2)?;
    let char4 = m.get(y + 3)?.get(x + 3)?;
    match (char1, char2, char3, char4) {
        ('X', 'M', 'A', 'S') => Some(true),
        _ => Some(false),
    }
}

fn check_diag_down_left(m: &Vec<Vec<char>>, x: usize, y: usize) -> Option<bool> {
    if x < 3 {
        return None;
    }

    let char1 = m.get(y)?.get(x)?;
    let char2 = m.get(y + 1)?.get(x - 1)?;
    let char3 = m.get(y + 2)?.get(x - 2)?;
    let char4 = m.get(y + 3)?.get(x - 3)?;
    match (char1, char2, char3, char4) {
        ('X', 'M', 'A', 'S') => Some(true),
        _ => Some(false),
    }
}

fn check_diag_up_right(m: &Vec<Vec<char>>, x: usize, y: usize) -> Option<bool> {
    if y < 3 {
        return None;
    }
    let char1 = m.get(y)?.get(x)?;
    let char2 = m.get(y - 1)?.get(x + 1)?;
    let char3 = m.get(y - 2)?.get(x + 2)?;
    let char4 = m.get(y - 3)?.get(x + 3)?;
    match (char1, char2, char3, char4) {
        ('X', 'M', 'A', 'S') => Some(true),
        _ => Some(false),
    }
}

fn check_diag_up_left(m: &Vec<Vec<char>>, x: usize, y: usize) -> Option<bool> {
    if x < 3 || y < 3 {
        return None;
    }
    let char1 = m.get(y)?.get(x)?;
    let char2 = m.get(y - 1)?.get(x - 1)?;
    let char3 = m.get(y - 2)?.get(x - 2)?;
    let char4 = m.get(y - 3)?.get(x - 3)?;
    match (char1, char2, char3, char4) {
        ('X', 'M', 'A', 'S') => Some(true),
        _ => Some(false),
    }
}
fn part1() {
    let file = std::fs::File::open("p4.in").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut input = std::fs::read_to_string("p4.in").unwrap();
    input.retain(|c| !c.is_whitespace());

    let mut m: Vec<Vec<char>> = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            let row = line.chars().collect::<Vec<char>>();
            m.push(row);
        }
    }

    let checkers = vec![
        check_up,
        check_right,
        check_down,
        check_left,
        check_diag_up_left,
        check_diag_up_right,
        check_diag_down_right,
        check_diag_down_left,
    ];

    let mut counts: usize = 0;
    for x in 0..m.len() {
        for y in 0..m.get(x).unwrap().len() {
            for checker in &checkers {
                let result = checker(&m, x, y);
                match result {
                    Some(true) => {
                        counts += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    println!("{counts}");
}

fn checker_x(m: &Vec<Vec<char>>, x: usize, y: usize) -> Option<bool> {
    if x < 1 || y < 1 {
        return None;
    }

    let c = m.get(y)?.get(x)?;
    let top_left = m.get(y - 1)?.get(x - 1)?;
    let top_right = m.get(y - 1)?.get(x + 1)?;
    let bot_left = m.get(y + 1)?.get(x - 1)?;
    let bot_right = m.get(y + 1)?.get(x + 1)?;

    match (c, top_left, top_right, bot_left, bot_right) {
        ('A', 'M', 'M', 'S', 'S') => Some(true),
        ('A', 'M', 'S', 'M', 'S') => Some(true),
        ('A', 'S', 'M', 'S', 'M') => Some(true),
        ('A', 'S', 'S', 'M', 'M') => Some(true),
        _ => None,
    }
}

fn part2() {
    let file = std::fs::File::open("p4.in").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut input = std::fs::read_to_string("p4.in").unwrap();
    input.retain(|c| !c.is_whitespace());

    let mut m: Vec<Vec<char>> = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            let row = line.chars().collect::<Vec<char>>();
            m.push(row);
        }
    }

    let mut counts: usize = 0;
    for x in 0..m.len() {
        for y in 0..m.get(x).unwrap().len() {
            let result = checker_x(&m, x, y);
            match result {
                Some(true) => {
                    counts += 1;
                }
                _ => {}
            }
        }
    }

    println!("{counts}");
}

fn main() {
    part2();
}
