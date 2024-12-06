use std::io::{self, BufRead};

fn part1() {
    let file = std::fs::File::open("p3.in").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut sum: usize = 0;

    for line in lines {
        println!("iterating lines");

        if let Ok(line) = line {
            println!("ok line");
            let mut split = line.split("mul");
            // skip first split before `mut`
            split.next();
            while let Some(s) = split.next() {
                println!("ok split: {}", s);
                let mut chars = s.chars();

                if s.len() < 5 {
                    println!("s.len < 5");
                    continue;
                }

                if chars.next().unwrap() != '(' {
                    println!("first char not (");
                    continue;
                }

                let mut last: Option<char> = None;
                let n1s = chars
                    .by_ref()
                    .take_while(|c| {last = Some(*c); c.is_digit(10)})
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
                    println!("found {} instead of ,", comma_char);
                    continue;
                }

                let n2s = chars
                    .by_ref()
                    .take_while(|c| {last = Some(*c); c.is_digit(10)})
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

                let mul = n1 * n2;
                sum += mul;
            }
        }
    }

    println!("total sum: {}", sum);
}

fn main() {
    part1();
}
