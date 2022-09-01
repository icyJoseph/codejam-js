use std::io;

fn is_palindrome(n: u64) -> bool {
    n.to_string().chars().rev().collect::<String>() == n.to_string()
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let num = parse_num::<u64>();

        let mut total = 0;
        let mut factor = 1;

        loop {
            if factor * factor > num {
                break;
            }

            if num % factor == 0 {
                if is_palindrome(factor) {
                    total = total + 1;
                }

                let comp = num / factor;

                if comp != factor && is_palindrome(comp) {
                    total = total + 1;
                }
            }

            factor = factor + 1;
        }

        println!("Case #{}: {}", case, total);
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
