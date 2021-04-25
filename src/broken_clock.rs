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

type Time = (Option<u128>, Option<u128>, Option<u128>, Option<u128>);

fn from_twelve(a: u128, b: u128, c: u128) -> Time {
    let base = (10u128).pow(9);

    // this means, a is the total number of nanosecs since midnight
    let _hours = a / (3600 * base);
    let _minutes = b / (12 * 60 * base);
    let _seconds = c / (720 * base);

    let expected_minutes = (a % (3600 * base) / (60 * base));

    let expected_seconds = (a % (3600 * base) % (60 * base)) / base;

    let hours = if _hours <= 11 { Some(_hours) } else { None };

    let minutes = if _minutes <= 59 && _minutes == expected_minutes {
        Some(_minutes)
    } else {
        None
    };

    let seconds = if _seconds <= 59 && _seconds == expected_seconds {
        Some(_seconds)
    } else {
        None
    };

    let _nano = if seconds.is_none() {
        None
    } else {
        Some((a % (3600 * base) % (60 * base)) % base)
    };

    (hours, minutes, seconds, _nano)
}

fn verify(head: Time) -> bool {
    if head.0.is_some() {
        if head.1.is_some() {
            if head.2.is_some() {
                return true;
            }
        }
    }
    return false;
}

fn search(spec: &Vec<u128>, permutation: &mut Vec<usize>, chosen: &mut Vec<bool>) -> Option<Time> {
    if permutation.len() == 3 {
        let a = spec[permutation[0]];
        let b = spec[permutation[1]];
        let c = spec[permutation[2]];

        let time = from_twelve(a, b, c);

        if verify(time) {
            return Some(time);
        } else {
            return None;
        }
    } else {
        for i in 0..3 {
            if chosen[i] {
                continue;
            }

            chosen[i] = true;
            permutation.push(i);
            let res = search(spec, permutation, chosen);
            if res.is_none() {
                chosen[i] = false;
                permutation.pop();
            } else {
                return res;
            }
        }
    }
    return None;
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<u128>();

        let mut chosen = vec![false; 6];
        let mut permutation: Vec<usize> = vec![];

        match search(&spec, &mut permutation, &mut chosen) {
            Some(time) => {
                let h = time.0.unwrap();
                let m = time.1.unwrap();
                let s = time.2.unwrap();
                let n = time.3.unwrap();
                println!("Case #{}: {} {} {} {}", case, h, m, s, n);
            }
            None => panic!("No time found"),
        }
    }
    Ok(())
}
