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

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let total = parse_num::<usize>();
        let song = parse_vec::<i64>();

        let mut streak_up = 0;
        let mut streak_down = 0;

        let mut rule_breaks = 0;

        for note in 1..total {
            let curr = song[note];
            let prev = song[note - 1];

            let next_edge = (curr - prev).signum();

            if next_edge == 1 {
                streak_up += 1;
                streak_down = 0;
            } else if next_edge == -1 {
                streak_down += 1;
                streak_up = 0;
            } else {
                continue;
            }

            if streak_up == 4 || streak_down == 4 {
                streak_up = 0;
                streak_down = 0;
                rule_breaks += 1;
            }
        }
        println!("Case #{}: {}", case, rule_breaks);
    }
    Ok(())
}
