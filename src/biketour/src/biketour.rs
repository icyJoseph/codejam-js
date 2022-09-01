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
        ptc::<usize>();

        let checks = vtc::<u32>();

        let mut total = 0;

        for i in 1..checks.len() - 1 {
            if checks[i] > checks[i - 1] && checks[i] > checks[i + 1] {
                total += 1;
            }
        }

        println!("Case #{}: {}", case, total);
    }
    Ok(())
}
