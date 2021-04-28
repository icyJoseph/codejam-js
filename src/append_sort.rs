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

fn mag(n: u32) -> u32 {
    let mut m = 1;

    loop {
        if n < ((10u32).pow(m)) {
            break;
        }
        m += 1
    }

    m
}

fn to_digits(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn to_number(digits: &Vec<u32>) -> u32 {
    digits.iter().fold(0, |prev, curr| 10 * prev + curr)
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let total = parse_num::<usize>();

        let mut list = parse_vec::<u32>();

        let mut changes = 0;

        for index in 1..total {
            let prev = list[index - 1];
            let curr = list[index];

            if curr > prev {
                continue;
            }

            let mut next = curr;
            let mut i = 0;
            let mut j = 1;

            loop {
                if i == (10u32).pow(j) {
                    i = 0;
                    j += 1;
                }
                next = curr * (10u32).pow(j) + i;
                i += 1;
                if next > prev {
                    changes += to_digits(next).len() - to_digits(curr).len();
                    list[index] = next;
                    break;
                }
            }
        }
        println!("Case #{}: {}", case, changes);
    }
    Ok(())
}
