use std::io;

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = nxt();
        let result = nxt();

        let spec_nodes = spec.trim().chars().collect::<Vec<_>>();

        let result_nodes = result.trim().chars().collect::<Vec<_>>();

        let mut skip = 0;

        let mut walker = result_nodes.iter();

        let mut pair_up = || {
            for target in spec_nodes.iter() {
                // move on result_nodes
                loop {
                    let next = walker.next();

                    match next {
                        Some(c) if target != c => {
                            // skip this character
                            skip += 1;
                        }
                        Some(_) => break,
                        None => {
                            return true;
                        }
                    }
                }
            }

            return false;
        };

        let impossible = pair_up();

        if impossible {
            println!("Case #{}: IMPOSSIBLE", case);
        } else {
            loop {
                match walker.next() {
                    Some(_) => {
                        skip += 1;
                    }
                    None => break,
                }
            }
            println!("Case #{}: {}", case, skip);
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
