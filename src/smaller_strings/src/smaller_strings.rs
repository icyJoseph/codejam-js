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

fn string_vec<T: std::string::ToString>(vec: &Vec<T>, separator: &str) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(separator)
}

fn to_val(vec: &Vec<i64>, k: i64) -> i64 {
    let mut total = 0;

    for i in 0..vec.len() {
        total += vec[i] * (k).pow((vec.len() - 1 - i) as u32);
    }

    total
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    let rec: Vec<char> = "0abcdefghijklmnopqrstuvwxyz".chars().collect();
    let m = 7 + (10_i64).pow(9);

    for case in 1..=n {
        let spec = parse_vec::<usize>();
        let limit = nxt();
        let source = rec[0..=spec[1]].to_vec();

        let rep = limit
            .trim()
            .chars()
            .map(|x| source.iter().position(|&c| c == x).unwrap())
            .map(|x| x as i64)
            .collect::<Vec<i64>>();

        let mut answer = 0;

        let base = if spec[0] % 2 == 0 {
            rep[0..spec[0] / 2].to_vec()
        } else {
            rep[0..=spec[0] / 2].to_vec()
        };

        let k = spec[1] as i64;

        for i in 0..base.len() {
            let digit = base[i];
            let exp = (base.len() - i) as u32;
            if i == base.len() - 1 {
                answer = (answer + digit) % m;
            } else {
                answer = (answer + (digit - 1) * ((k).pow(exp - 1))) % m;
            }
        }

        let mut max = base.clone();

        let mut cmp = if spec[0] % 2 == 0 {
            base[..].to_vec()
        } else {
            base[0..(base.len() - 1)].to_vec()
        };

        cmp.reverse();

        max.append(&mut cmp);
        if to_val(&max, k) >= to_val(&rep, k) {
            answer -= 1;
        }

        println!("Case #{}: {}", case, answer);
    }
    Ok(())
}
