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
        let spec = parse_num::<u32>();
        let string = nxt();

        let chars = string.trim().chars();

        let mut sub = vec![];
        let mut run = 1;

        let mut prev = None;

        for (i, c) in chars.enumerate() {
            if i == 0 {
                sub.push(1);
                prev = Some(c);
                continue;
            }

            if c > prev.unwrap() {
                run += 1;
            } else {
                run = 1;
            }

            sub.push(run);
            prev = Some(c);
        }

        let result = sub
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        println!("Case #{}: {}", case, result);
    }
    Ok(())
}
