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

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let _q = parse_num::<usize>();

        let mut list = parse_vec::<u64>();

        let mut sorted = list.clone();

        sorted.sort();

        let mut cost = 0;

        for (i, entry) in sorted.iter().enumerate() {
            let pos = list.iter().position(|x| x == entry).unwrap();

            if i == _q - 1 {
                break;
            }

            let rev: Vec<u64> = list[i..=pos].iter().rev().map(|&x| x).collect();
            cost += pos - i + 1;

            for (x, &el) in rev.iter().enumerate() {
                list[x + i] = el;
            }
        }

        println!("Case #{}: {}", case, cost);
    }
    Ok(())
}
