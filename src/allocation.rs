use std::io;

type Res<T> = Result<T, Box<dyn std::error::Error>>;

fn nxt() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        _ => panic!("Error reading line"),
    }
}

fn ptc<T: std::str::FromStr>() -> T {
    match nxt().trim().parse::<T>() {
        Ok(n) => n,
        _ => panic!("Error parsing"),
    }
}

fn vtc<T: std::str::FromStr>() -> Vec<T> {
    nxt()
        .split_whitespace()
        .map(|x| match x.parse::<T>() {
            Ok(n) => n,
            _ => panic!("Could not parse vector"),
        })
        .collect()
}

fn main() -> Res<()> {
    let n = ptc::<u32>();
    for case in 1..=n {
        let spec = vtc::<u64>();

        let mut budget = spec[1];

        let mut prices = vtc::<u64>();

        prices.sort();

        let mut index = 0;

        loop {
            if index == prices.len() {
                break;
            }

            let next = prices[index];

            if budget < next {
                break;
            }

            budget -= next;
            index += 1;
        }

        println!("Case #{}: {}", case, index);
    }
    Ok(())
}
