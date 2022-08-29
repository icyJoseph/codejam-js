use std::io;

enum Direction {
    N,
    S,
    W,
    E,
}

fn move_robot(y: usize, x: usize, direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::N => (y - 1, x),
        Direction::S => (y + 1, x),
        Direction::W => (y, x - 1),
        Direction::E => (y, x + 1),
    }
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<usize>();

        let cols = spec[2];
        let start_r = spec[3] - 1;
        let start_c = spec[4] - 1;

        let steps_raw = nxt();

        let steps = steps_raw
            .trim()
            .chars()
            .map(|s| match s {
                'N' => Direction::N,
                'S' => Direction::S,
                'W' => Direction::W,
                'E' => Direction::E,
                _ => panic!("Invalid direction"),
            })
            .collect::<Vec<Direction>>();

        use std::collections::HashSet;

        let mut grid = HashSet::new();
        let mut current = (start_r, start_c);

        grid.insert(start_r * cols + start_c);

        for step in steps.iter() {
            while grid.contains(&(current.0 * cols + current.1)) {
                current = move_robot(current.0, current.1, step);
            }

            grid.insert(current.0 * cols + current.1);
        }

        println!("Case #{}: {} {}", case, current.0 + 1, current.1 + 1);
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
