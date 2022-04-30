use std::io;

fn is_valid(string: &str) -> bool {
    use std::collections::HashMap;
    let mut table: HashMap<char, Option<usize>> = HashMap::new();

    let chars = string.chars().collect::<Vec<char>>();

    for (i, &c) in chars.iter().enumerate() {
        let current = table.entry(c).or_insert(None);

        match current {
            None => {
                *current = Some(i);
            }
            Some(j) => {
                if *j + 1 != i {
                    return false;
                }
                *current = Some(i)
            }
        }
    }

    return true;
}

fn construct(input: String, others: Vec<String>) -> Option<String> {
    if others.len() == 0 {
        return if is_valid(&input) { Some(input) } else { None };
    }

    for (i, word) in others.iter().enumerate() {
        let next = format!("{}", word);
        let after_others = others
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, w)| w.to_string())
            .collect::<Vec<String>>();
        // put the next one in others, to front

        let fwd_input = format!("{}{}", input, next);
        let fwd = construct(fwd_input, after_others);

        match fwd {
            Some(result) if is_valid(&result) => return Some(result),
            _ => {}
        }

        // put the next one in others, to the back
        let after_others = others
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, w)| w.to_string())
            .collect::<Vec<String>>();
        let back_input = format!("{}{}", next, input);
        let back = construct(back_input, after_others);

        match back {
            Some(result) if is_valid(&result) => return Some(result),
            _ => {}
        }
    }
    return None;
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let _total = parse_num::<usize>();
        let towers = parse_vec::<String>();

        if towers.iter().any(|t| !is_valid(t)) {
            println!("Case #{}: {}", case, "IMPOSSIBLE");
        } else {
            let start = format!("{}", towers[0]);
            let others = towers[1..].to_vec();
            let result = construct(start, others);

            match result {
                Some(r) => {
                    println!("Case #{}: {}", case, r);
                }
                _ => {
                    println!("Case #{}: {}", case, "IMPOSSIBLE");
                }
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
