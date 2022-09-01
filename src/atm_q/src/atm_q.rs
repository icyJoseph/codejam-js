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

        let total = spec[0] as usize;
        let max = spec[1];

        let amounts = parse_vec::<u32>();

        let mut exit: Vec<(usize, u32)> = (0..total)
            .collect::<Vec<usize>>()
            .iter()
            .map(|&x| {
                let qty = amounts[x];

                let times = if qty % max == 0 {
                    qty / max
                } else {
                    qty / max + 1
                };

                (x, times)
            })
            .collect::<Vec<(usize, u32)>>();

        exit.sort_by(|a, b| a.1.cmp(&b.1));

        let answer = format!(
            "{}",
            exit.iter()
                .map(|x| (x.0 + 1).to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );

        println!("Case #{}: {}", case, answer);
    }
    Ok(())
}
