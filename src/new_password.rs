use std::io;

#[derive(Debug)]
enum Policy {
    Length(usize),
    UpperCase,
    LowerCase,
    Digit,
    Special,
}

fn policy(pwd: &Vec<char>) -> Vec<Policy> {
    let mut faults: Vec<Policy> = vec![];

    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_digit = false;
    let mut has_special = false;

    for c in pwd {
        if c.is_uppercase() {
            has_uppercase = true;
        }
        if c.is_lowercase() {
            has_lowercase = true;
        }

        if c.is_digit(10) {
            has_digit = true;
        }

        match c {
            '#' | '@' | '*' | '&' => {
                has_special = true;
            }
            _ => {}
        }
    }

    let mut min_extension = 0;

    if !has_uppercase {
        faults.push(Policy::UpperCase);
        min_extension += 1;
    }

    if !has_lowercase {
        faults.push(Policy::LowerCase);
        min_extension += 1;
    }

    if !has_digit {
        faults.push(Policy::Digit);
        min_extension += 1;
    }

    if !has_special {
        faults.push(Policy::Special);
        min_extension += 1;
    }

    if pwd.len() < (7 - min_extension) {
        faults.push(Policy::Length(7 - min_extension - pwd.len()));
    }

    faults
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let _len = parse_num::<usize>();

        let raw_pwd = nxt();
        let pwd = raw_pwd.trim().chars().collect::<Vec<char>>();

        let mut new_pwd = String::from(raw_pwd.trim());

        let result = policy(&pwd);

        for fault in result {
            match fault {
                Policy::Digit => {
                    new_pwd = format!("{}{}", new_pwd, "0");
                }
                Policy::Special => {
                    new_pwd = format!("{}{}", new_pwd, "&");
                }
                Policy::LowerCase => {
                    new_pwd = format!("{}{}", new_pwd, "a");
                }
                Policy::UpperCase => {
                    new_pwd = format!("{}{}", new_pwd, "A");
                }
                Policy::Length(n) => {
                    new_pwd = format!("{}{}", new_pwd, string_vec(&vec!['A'; n], ""));
                }
            }
        }

        println!("Case #{}: {}", case, new_pwd);
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
