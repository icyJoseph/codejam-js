use std::io;

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let bounds = parse_vec::<u64>();

        let lb = bounds[0];
        let up = bounds[1];

        let naive = |num: u64| {
            let num_str = num.to_string();
            let digits = num_str
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();

            let mut sum: u32 = digits.iter().sum();

            for &d in digits.iter() {
                if d == 0 {
                    return true;
                }

                sum = sum / gcd(sum, d);

                if sum == 1 {
                    break;
                }
            }

            sum == 1
        };

        let mut total = 0;

        for next in lb..=up {
            if naive(next) {
                total += 1;
            }
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
