use std::io;

// shortest difference mod limit
fn diff(dial: i64, other: i64, limit: i64) -> i64 {
    // 1 9 10 -> 2
    std::cmp::min(
        (dial - other).rem_euclid(limit),
        (other - dial).rem_euclid(limit),
    )
}

fn abs_diff(dial: i64, other: i64, limit: i64) -> i64 {
    diff(dial, other, limit).abs()
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<i64>();

        // let total = spec[0];
        let dial = spec[1];

        let source = parse_vec::<i64>();

        let total = source.len();

        let mut best = None;
        let mut s_left = 0;
        let mut s_right = 0;

        loop {
            let mut locks = source.to_vec();
            let mut rotations = 0;

            let mut left: usize = s_left;
            let mut right: usize = s_right;

            if s_left == total || s_right == total {
                break;
            }

            if locks[s_left] != locks[s_right] {
                // skip
                if s_left == s_right {
                    s_right = s_right + 1;
                } else {
                    s_left = s_left + 1;
                }
                continue;
            }

            loop {
                if locks.iter().all(|&x| x == 0) {
                    break;
                }

                if left > 0 && locks[left - 1] == locks[left] {
                    // expand to cover left
                    left = left - 1;
                } else if right < total - 1 && locks[right + 1] == locks[right] {
                    // expand to cover right
                    right = right + 1;
                } else if left > 0 && right < total - 1 {
                    if abs_diff(locks[left], locks[left - 1], dial)
                        < abs_diff(locks[right], locks[right + 1], dial)
                    {
                        // rotate as many times as needed to match the left
                        rotations += abs_diff(locks[left], locks[left - 1], dial);
                        for i in left..=right {
                            locks[i] = locks[left - 1];
                        }
                        left = left - 1;
                    } else {
                        // rotate as many time as needed to match the right
                        rotations += abs_diff(locks[right], locks[right + 1], dial);
                        for i in left..=right {
                            locks[i] = locks[right + 1];
                        }
                        right = right + 1;
                    }
                } else if left > 0 {
                    // expand left
                    rotations += abs_diff(locks[left], locks[left - 1], dial);
                    for i in left..=right {
                        locks[i] = locks[left - 1];
                    }
                    left = left - 1;
                } else if right < total - 1 {
                    // expand right
                    rotations += abs_diff(locks[right], locks[right + 1], dial);
                    for i in left..=right {
                        locks[i] = locks[right + 1];
                    }
                    right = right + 1;
                } else {
                    // left and right have reached the borders
                    rotations += abs_diff(locks[0], 0, dial);

                    for i in left..=right {
                        locks[i] = 0;
                    }

                    break;
                }
            }

            match best {
                Some(n) if rotations > n => {}
                _ => best = Some(rotations),
            }

            if s_left == s_right {
                s_right = s_right + 1;
            } else {
                s_left = s_left + 1;
            }
        }

        match best {
            None => {
                println!("Case #{}: {:?}", case, source);
            }
            Some(b) => {
                println!("Case #{}: {}", case, b);
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
