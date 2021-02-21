use std::io;

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

fn parse_vec<T: std::str::FromStr>() -> Vec<T> {
    nxt()
        .split_whitespace()
        .map(|x| match x.parse::<T>() {
            Ok(n) => n,
            _ => panic!("Could not parse vector"),
        })
        .collect()
}

fn ceil(a: u32, b: u32) -> u32 {
    let rest = if a % b == 0 { 0 } else { 1 };
    a / b + rest
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<u32>();

        let total = spec[0];

        let max = spec[1];

        let mut intervals = vec![];

        for _ in 0..total {
            let entry = parse_vec::<u32>();
            intervals.push((entry[0], entry[1]));
        }

        intervals.sort_by(|a, b| a.0.cmp(&b.0));

        let mut position = 0;
        let mut robots = 0;

        for (head, tail) in intervals {
            if tail <= position {
                continue;
            }
            let start = std::cmp::max(head, position);
            let new_robots = ceil(tail - start, max);
            robots += new_robots;
            position = start + new_robots * max;
        }

        println!("Case #{}: {}", case, robots);
    }
    Ok(())
}
