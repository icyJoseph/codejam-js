use std::io;

use std::f64::consts::PI;

enum Direction {
    LEFT,
    RIGHT,
}

impl Direction {
    fn toggle(&mut self) {
        match self {
            Direction::LEFT => {
                *self = Direction::RIGHT;
            }
            Direction::RIGHT => {
                *self = Direction::LEFT;
            }
        }
    }
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<u64>();

        let a = spec[1];
        let b = spec[2];

        let mut current = spec[0];
        let mut total_area = 0;
        let mut dir = Direction::RIGHT;

        loop {
            if current == 0 {
                break;
            }

            total_area = total_area + (current).pow(2u32);

            match dir {
                Direction::LEFT => {
                    current = current / b;
                }
                Direction::RIGHT => {
                    current = current * a;
                }
            }

            dir.toggle();
        }

        println!("Case #{}: {:.6}", case, (total_area as f64) * PI);
    }

    Ok(())
}

type Res<T> = Result<T, Box<dyn std::error::Error>>;

fn nxt() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        _ => panic!("Error reading line"),
    }
}

fn parse_num<T: std::str::FromStr>() -> T {
    match nxt().trim().parse::<T>() {
        Ok(n) => n,
        _ => panic!("Error parsing"),
    }
}

#[allow(dead_code)]
fn parse_vec<T: std::str::FromStr>() -> Vec<T> {
    nxt()
        .split_whitespace()
        .map(|x| match x.parse::<T>() {
            Ok(n) => n,
            _ => panic!("Could not parse vector"),
        })
        .collect()
}

#[allow(dead_code)]
fn string_vec<T: std::string::ToString>(vec: &Vec<T>, separator: &str) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(separator)
}
