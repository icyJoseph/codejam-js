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

fn range_primes(start: u64, end: u64, target: u64) -> u64 {
    let mut lower = start;
    let mut upper = end;

    let mut primes = vec![];

    if lower <= 2 {
        lower = 2;
        if upper >= 2 {
            primes.push(lower);
            lower += 1;
        }
    }

    if lower % 2 == 0 {
        lower += 1;
    }

    let mut num = lower;

    loop {
        let mut is_prime = true;

        let mut factor = 2;

        loop {
            if num % factor == 0 {
                is_prime = false;
                break;
            }

            if factor * factor > num {
                break;
            }

            factor += 1;
        }

        if is_prime {
            let len = primes.len();
            if len > 1 {
                let res = num * primes[len - 1];

                if res > target {
                    return primes[len - 1] * primes[len - 2];
                }
            }

            primes.push(num);
        }

        num += 2;
    }
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let target = parse_num::<u64>();

        let base = (target as f64).sqrt() as u64;

        let lower = if base > 100000 { base - 999 } else { base / 4 };
        let upper = if base > 100000 { base + 999 } else { base };
        let answer = range_primes(lower, upper, target);

        println!("Case #{}: {}", case, answer);
    }
    Ok(())
}
