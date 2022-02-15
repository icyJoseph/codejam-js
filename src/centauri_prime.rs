use std::io;

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let line = nxt();
        let kingdom = line.trim();

        match kingdom.to_lowercase().chars().last() {
            Some(n) if ['a', 'e', 'i', 'o', 'u'].contains(&n) => {
                println!("Case #{}: {} is ruled by {}.", case, kingdom, "Alice");
            }
            Some('y') => {
                println!("Case #{}: {} is ruled by {}.", case, kingdom, "nobody");
            }
            _ => {
                println!("Case #{}: {} is ruled by {}.", case, kingdom, "Bob");
            }
        }
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
