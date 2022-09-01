use std::io;

fn is_palindrome(base: &Vec<char>, extension: &[char]) -> bool {
    let base_len = base.len();
    let ext_len = extension.len();
    let len = base_len + ext_len;

    for i in 0..len / 2 {
        let right_index = len - 1 - i;

        let right = if right_index >= base_len {
            extension[right_index - base_len]
        } else {
            base[right_index]
        };

        if base[i] != right {
            return false;
        }
    }

    true
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let len = parse_num::<usize>();

        let raw = nxt();
        let p = raw.trim().chars().collect::<Vec<char>>();

        for index in 0..len {
            if is_palindrome(&p, &p[..index + 1]) {
                println!(
                    "Case #{}: {}",
                    case,
                    &p[..index + 1]
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                );
                break;
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
