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

fn string_vec<T: std::string::ToString>(vec: &Vec<T>, separator: &str) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(separator)
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let g = parse_num::<u64>();

        let mut count = 0;
        let mut day = 1;
        loop {
            if day * (day + 1) > 2 * g {
                break;
            }

            if (g - day * (day + 1) / 2) % day == 0 {
                count += 1;
            }
            day += 1;
        }
        println!("Case #{}: {}", case, count);
    }
    Ok(())
}
