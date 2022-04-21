use std::io;

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let qty = parse_num::<u32>();

        let mut mine = {
            let mut i = 0;
            let mut ret: Vec<u64> = vec![];

            loop {
                if i == 30 {
                    break;
                }

                ret.push((2_u64).pow(29 - i));

                i = i + 1;
            }

            let mut j: u64 = 1;

            while ret.len() < (qty as usize) {
                if !(2 * j).is_power_of_two() {
                    ret.push(2 * j);
                }
                j = j + 1;
            }

            ret
        };

        mine.sort_by(|a, b| a.cmp(&b).reverse());

        println!("{}", string_vec(&mine, " "));

        let mut judge = parse_vec::<u64>();

        judge.sort_by(|a, b| a.cmp(&b).reverse());

        let mut left = vec![];
        let mut right = vec![];

        for i in judge {
            if left.iter().sum::<u64>() < right.iter().sum::<u64>() {
                left.push(i);
            } else {
                right.push(i);
            }
        }

        for i in mine {
            if left.iter().sum::<u64>() < right.iter().sum::<u64>() {
                left.push(i);
            } else {
                right.push(i);
            }
        }

        println!("{}", string_vec(&left, " "));
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
