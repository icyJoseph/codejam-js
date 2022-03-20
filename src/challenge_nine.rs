use std::io;

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = nxt();
        let digits_ch = spec.trim().chars().collect::<Vec<_>>();

        let mut digits = digits_ch
            .iter()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        let mut target = 0;

        for d in digits.iter() {
            target = (target + d) % 9;
        }

        let stub = if target != 0 { 9 - target } else { 0 };

        let mut right = 0;

        loop {
            if right == 0 && stub == 0 {
                right += 1;
                continue;
            }

            if right == digits.len() {
                digits.push(stub);
                break;
            }

            let next = digits[right];

            if stub < next {
                digits.insert(right, stub);
                break;
            }

            right += 1;
        }

        let result = string_vec(&digits, "");

        println!("Case #{}: {}", case, result);
    }

    Ok(())
}

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

#[allow(dead_code)]
fn parse_vec<T: std::str::FromStr>() -> Vec<T> {
    nxt()
        .split_whitespace()
        .map(|x| match x.parse::<T>() {
            Ok(n) => n,
            _ => panic!("Could not parse vector"),
        })
        .collect()
}

#[allow(dead_code)]
fn string_vec<T: std::string::ToString>(vec: &Vec<T>, separator: &str) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(separator)
}
