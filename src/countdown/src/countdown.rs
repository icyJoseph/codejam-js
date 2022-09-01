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
        let spec = parse_vec::<u64>();

        let arr = parse_vec::<u64>();

        let k = spec[1];

        let mut count = 0;

        let mut len = 0;

        for (i, &num) in arr.iter().enumerate() {
            if num == k {
                len = 1;
            } else {
                if len == k - 1 && num == 1 {
                    count += 1;
                    len = 0;
                } else if k > num && len == k - num && num + 1 == arr[i - 1] {
                    len += 1;
                } else {
                    len = 0;
                }
            }
        }

        println!("Case #{}: {}", case, count);
    }
    Ok(())
}
