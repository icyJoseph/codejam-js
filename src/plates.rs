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

fn sum(src: &Vec<u32>, to: usize) -> u32 {
    src[..=to].iter().sum()
}

fn main() -> Res<()> {
    let n = ptc::<u32>();
    for case in 1..=n {
        let spec = vtc::<usize>();

        let total = spec[0];
        let stack_len = spec[1];
        let req = spec[2];

        let mut stacks = vec![];

        for _ in 0..total {
            let stack = vtc::<u32>();
            stacks.push(stack);
        }

        let mut acc: Vec<Vec<u32>> = vec![vec![0; req + 1]; total + 1];

        use std::cmp::{max, min};

        for stack in 1..=total {
            for selection in 1..=req {
                for plate in 0..min(min(stack_len, selection), req) {
                    acc[stack][selection] = max(
                        acc[stack][selection],
                        max(
                            acc[stack - 1][selection],
                            sum(&stacks[stack - 1], plate) + acc[stack - 1][selection - plate - 1],
                        ),
                    );
                }
            }
        }

        println!("Case #{}: {}", case, acc[total][req]);
    }
    Ok(())
}
