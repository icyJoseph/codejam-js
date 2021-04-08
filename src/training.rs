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
        let spec = parse_vec::<u32>();
        let team_size = spec[1];

        let mut skills = parse_vec::<u32>();

        skills.sort_by(|a, b| a.cmp(b));

        let mut hours: Option<u32> = None;

        let size = team_size as usize;

        for i in 0..=skills.len() - size {
            let team = &skills[i..i + size];
            let last = team.iter().last().unwrap();
            let cost: u32 = team.iter().map(|x| last - x).sum();

            match hours {
                None => {
                    hours = Some(cost);
                }
                Some(c) => {
                    if cost < c {
                        hours = Some(cost);
                    }
                }
            }
        }

        let min = match hours {
            None => 0,
            Some(c) => c,
        };

        println!("Case #{}: {}", case, min);
    }
    Ok(())
}
