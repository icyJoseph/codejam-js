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

type Time = (u128, u128, u128, u128);

const NS: u128 = 1_000_000_000;
const ONE_REV: u128 = 3600 * 12 * NS;

fn to_rep(time: u128) -> Time {
    let hours = time / 3600;
    let minutes = (time % 3600) / 60;
    let seconds = (time % 3600) % 60;
    let nano = ((time % 3600) % 60) / NS;

    (hours, minutes, seconds, nano)
}

fn normalize(src: &Vec<u128>) -> Vec<u128> {
    vec![
        0,
        (src[1] + ONE_REV - src[0]) % ONE_REV,
        (src[2] + ONE_REV - src[1]) % ONE_REV,
    ]
}

fn to_ticks(second: u128) -> Vec<u128> {
    let total_ns = NS * second;

    let ret = vec![
        total_ns % ONE_REV,
        12 * total_ns % ONE_REV,
        720 * total_ns % ONE_REV,
    ];

    normalize(&ret)
}

fn time_dict() -> Vec<Vec<u128>> {
    let mut ticks = vec![];

    for second in 0..12 * 60 * 60 {
        let as_ticks = to_ticks(second);
        ticks.push(as_ticks);
    }

    ticks
}

fn search(
    spec: &Vec<u128>,
    permutation: &mut Vec<usize>,
    chosen: &mut Vec<bool>,
    reps: &Vec<Vec<u128>>,
) -> Option<usize> {
    if permutation.len() == 3 {
        let a = spec[permutation[0]];
        let b = spec[permutation[1]];
        let c = spec[permutation[2]];

        let d = normalize(&vec![a, b, c]);

        return reps.iter().position(|rep| *rep == d);
    } else {
        for i in 0..3 {
            if chosen[i] {
                continue;
            }

            chosen[i] = true;
            permutation.push(i);

            let res = search(spec, permutation, chosen, reps);

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

    let reps = time_dict();

    for case in 1..=n {
        let spec = parse_vec::<u128>();

        let mut chosen = vec![false; 6];
        let mut permutation: Vec<usize> = vec![];

        match search(&spec, &mut permutation, &mut chosen, &reps) {
            Some(time) => {
                let (h, m, s, n) = to_rep(time as u128);
                println!("Case #{}: {} {} {} {}", case, h, m, s, n);
            }
            None => {
                println!("Case #{}: _", case);
            }
        }
    }

    Ok(())
}
