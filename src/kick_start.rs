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
        let mut frag: Vec<char> = nxt().trim().chars().rev().collect();

        let mut carry = 0;
        let mut total = 0;

        for (i, word) in frag.iter().enumerate() {
            let kick = ['K', 'C', 'I', 'K'];
            let start = ['T', 'R', 'A', 'T', 'S'];
            match word {
                'S' => {
                    if i >= 4 {
                        if &frag[i - 4..=i] == start {
                            carry += 1;
                        }
                    }
                }
                'K' => {
                    if i >= 3 {
                        if &frag[i - 3..=i] == kick {
                            total += carry;
                        }
                    }
                }
                _ => {
                    continue;
                }
            }
        }

        println!("Case #{}: {}", case, total);
    }
    Ok(())
}
