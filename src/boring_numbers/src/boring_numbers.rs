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

fn count_digits(num: u64) -> u32 {
    num.to_string().chars().map(|_| 1).sum()
}

fn trailing_zeroes(num: u64) -> u32 {
    let mut zeroes = 0;
    let mut pivot = num;

    while pivot % 10 == 0 {
        pivot = pivot / 10;
        zeroes += 1;
    }
    zeroes
}

fn digit_check(num: u64, skip_last: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    for (i, digit) in digits.iter().enumerate() {
        // even position

        if (skip_last as usize) + i > digits.len() - 1 {
            break;
        }
        if (i + 1) % 2 == 0 {
            if digit % 2 != 0 {
                return false;
            }
        } else {
            // odd position
            if digit % 2 == 0 {
                return false;
            }
        }
    }

    return true;
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();

    for case in 1..=n {
        let range = parse_vec::<u64>();

        let s = range[0];
        let e = range[1];
        let mut total = 0;

        if e - s < 10 {
            for number in s..=e {
                if digit_check(number, 0) {
                    total += 1;
                }
            }
            println!("Case #{}: {}", case, total);
        } else {
            // divide in groups

            let delta_start = 10 - s % 10;

            let start = s + if delta_start == 10 { 0 } else { delta_start };
            let end = e - (e % 10);

            let mut interval = start;

            loop {
                if interval >= end {
                    break;
                }

                let zeroes = trailing_zeroes(interval);

                let mut step = (10_u64).pow(zeroes);

                while step + interval > end {
                    if step > 10 {
                        step = step / 10;
                    } else {
                        break;
                    }
                }

                let exp = count_digits(step) - 1;
                if digit_check(interval, exp) {
                    total += (5_u64).pow(exp);
                }

                interval += step;
            }

            for number in s..start {
                if digit_check(number, 0) {
                    total += 1;
                }
            }

            for number in std::cmp::max(s, end)..=e {
                if digit_check(number, 0) {
                    total += 1;
                }
            }

            println!("Case #{}: {}", case, total);
        }
    }
    Ok(())
}
