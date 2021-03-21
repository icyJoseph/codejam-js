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
        let spec = parse_vec::<u32>();
        let string = nxt();

        let length = spec[0] as usize;
        let K = spec[1];

        let chars_it = string.trim().chars().collect::<Vec<char>>();

        let mut raw_k = 0;

        for (index, &ch) in chars_it.iter().enumerate() {
            let z = index + 1;
            let z_c = length - z + 1;

            if z > length / 2 {
                break;
            }

            if chars_it[z - 1] != chars_it[z_c - 1] {
                raw_k += 1;
            }
        }

        if raw_k == K {
            println!("Case #{}: {}", case, 0);
        } else {
            if raw_k > K {
                println!("Case #{}: {}", case, raw_k - K);
            } else {
                println!("Case #{}: {}", case, K - raw_k);
            }
        }
    }
    Ok(())
}
