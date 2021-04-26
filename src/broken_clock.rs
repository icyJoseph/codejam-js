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

    (hours, minutes, seconds, 0)
}

fn normalize(vec: &Vec<u128>) -> Vec<u128> {
    let mut ret = vec.to_vec();
    ret.sort_by(|a, b| a.cmp(&b));

    let one = (ONE_REV - ret[2]) + ret[0];
    let two = ret[1] - ret[0];
    let three = ret[2] - ret[1];

    let mut angles = vec![one, two, three];

    angles.sort_by(|a, b| a.cmp(&b));

    angles
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

fn main() -> Res<()> {
    let n = parse_num::<u32>();

    let reps = time_dict();

    for case in 1..=n {
        let spec = parse_vec::<u128>();

        let order_spec = normalize(&spec);

        match reps.iter().position(|rep| {
            rep[0] == order_spec[0] && rep[1] == order_spec[1] && rep[2] == order_spec[2]
        }) {
            Some(time) => {
                let (h, m, s, n) = to_rep(time as u128);
                println!("Case #{}: {} {} {} {}", case, h, m, s, n);
            }
            None => panic!("No time found"),
        }
    }
    Ok(())
}
