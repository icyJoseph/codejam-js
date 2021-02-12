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
        let total = parse_num::<usize>();
        let days = parse_vec::<u64>();

        let mut max = days[0];
        let mut count = 0;

        if total > 1 {
            if max > days[1] {
                count += 1;
            }
        } else {
            count += 1;
        }

        for day in 1..total {
            let current = days[day];

            if current > max {
                max = current;

                if day == total - 1 {
                    count += 1;
                } else if current > days[day + 1] {
                    count += 1;
                }
            }
        }

        println!("Case #{}: {}", case, count);
    }
    Ok(())
}
