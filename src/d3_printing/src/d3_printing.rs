use std::io;

const REQ_INK: u64 = 1_000_000;

// let d = c + m + y + k
fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let first_printer = parse_vec::<u64>();
        let second_printer = parse_vec::<u64>();
        let third_printer = parse_vec::<u64>();

        let c_opts = vec![first_printer[0], second_printer[0], third_printer[0]];
        let m_opts = vec![first_printer[1], second_printer[1], third_printer[1]];
        let y_opts = vec![first_printer[2], second_printer[2], third_printer[2]];
        let k_opts = vec![first_printer[3], second_printer[3], third_printer[3]];

        let mut min_c = *c_opts.iter().min().unwrap();
        let mut min_m = *m_opts.iter().min().unwrap();
        let mut min_y = *y_opts.iter().min().unwrap();
        let mut min_k = *k_opts.iter().min().unwrap();

        let min_total = min_c + min_m + min_y + min_k;

        if min_total < REQ_INK {
            println!("Case #{}: IMPOSSIBLE", case);
        } else {
            let mut overshoot = min_total - REQ_INK;

            loop {
                if overshoot == 0 {
                    break;
                }

                if min_c >= overshoot {
                    min_c = min_c - overshoot;
                    overshoot = 0;
                } else if min_y >= overshoot {
                    min_y = min_y - overshoot;
                    overshoot = 0;
                } else if min_m >= overshoot {
                    min_m = min_m - overshoot;
                    overshoot = 0;
                } else if min_k >= overshoot {
                    min_k = min_k - overshoot;
                    overshoot = 0;
                } else {
                    // overshoot is bigger than any single ink
                    // zero one ink at a time
                    if min_c > 0 {
                        overshoot = overshoot - min_c;
                        min_c = 0;
                    } else if min_m > 0 {
                        overshoot = overshoot - min_m;
                        min_m = 0;
                    } else if min_y > 0 {
                        overshoot = overshoot - min_y;
                        min_y = 0;
                    } else if min_k > 0 {
                        overshoot = overshoot - min_k;
                        min_k = 0;
                    }
                }
            }

            println!("Case #{}: {} {} {} {}", case, min_c, min_m, min_y, min_k);
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
