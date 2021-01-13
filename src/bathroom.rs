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

fn split(n: u64) -> (u64, u64) {
    if n % 2 == 0 {
        return (n / 2 - 1, n / 2);
    }
    return (n / 2, n / 2);
}

fn main() -> Res<()> {
    let n = ptc::<i32>();

    for case in 1..=n {
        let entry: Vec<u64> = nxt()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let s = entry[0];
        let k = entry[1];

        use std::collections::BinaryHeap;

        let mut set: BinaryHeap<u64> = BinaryHeap::new();

        set.push(s);

        let mut l = 0;
        let mut r = 0;

        for _ in 0..k {
            let next = set.pop().unwrap();
            let sp = split(next);

            l = sp.0;
            r = sp.1;

            set.push(sp.0);
            set.push(sp.1);
        }

        println!(
            "Case #{}: {} {}",
            case,
            std::cmp::max(l, r),
            std::cmp::min(l, r)
        );
    }
    
    Ok(())
}
