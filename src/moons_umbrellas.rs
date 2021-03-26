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
        let line = nxt();

        let spec = line
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let x = spec[0].parse::<i64>().unwrap();
        let y = spec[1].parse::<i64>().unwrap();

        let mural = spec[2].to_string();

        let char_vec: Vec<char> = mural.chars().collect();

        let mut result = vec![];

        if char_vec.len() == 1 {
            println!("Case #{}: {}", case, 0);
            continue;
        }

        for (i, &c) in char_vec.iter().enumerate() {
            if i == 0 {
                if c == '?' {
                    let mut di = 0;
                    let mut next = char_vec[1 + di];

                    while next == '?' {
                        di += 1;
                        if 1 + di == char_vec.len() - 1 {
                            next = 'C';
                            break;
                        }
                        next = char_vec[1 + di];
                    }

                    result.push(next);
                } else {
                    result.push(c);
                }
            } else {
                if c == '?' {
                    let prev = result[i - 1];
                    result.push(prev);
                } else {
                    result.push(c);
                }
            }
        }

        let mut cost = 0;

        for (i, &entry) in result.iter().enumerate() {
            if i == result.len() - 1 {
                break;
            }
            let next = result[i + 1];
            if entry == 'C' {
                if next == 'J' {
                    cost += x;
                }
            } else if entry == 'J' {
                if next == 'C' {
                    cost += y;
                }
            }
        }

        println!("Case #{}: {}", case, cost);
    }
    Ok(())
}
