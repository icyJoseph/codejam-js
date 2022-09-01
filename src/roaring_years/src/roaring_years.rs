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

fn to_digits(num: u32) -> Vec<u32> {
    num.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

fn add_one(vec: &Vec<u32>) -> u32 {
    1 + vec.iter().fold(0, |p, c| p * 10 + c)
}

fn is_roaring(vec: &Vec<u32>) -> bool {
    let mut slice = 0;
    let mut p_slice = slice;
    let mut pivot = 0;

    loop {
        let curr = vec[pivot..=slice].to_vec();

        let next = add_one(&curr);

        let next_digits = to_digits(next);
        let len = next_digits.len();

        if slice + len >= vec.len() {
            // can't slice enough to meet next
            break;
        }

        let actual = vec[slice + 1..=slice + len].to_vec();

        if actual == next_digits {
            // the next set of digits worked
            // and was the last
            if slice + len == vec.len() - 1 {
                return true;
            }
            pivot = slice + 1;
            slice = slice + len;
        } else {
            slice = p_slice + 1;
            pivot = 0;
            p_slice = slice;

            if pivot + slice >= vec.len() - 1 {
                break;
            }
        }
    }

    return false;
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let year = nxt();
        let mut digits = year
            .trim()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        if is_roaring(&digits) {
            digits = to_digits(add_one(&digits));
        }

        loop {
            if is_roaring(&digits) {
                break;
            }
            digits = to_digits(add_one(&digits));
        }
        let res = string_vec(&digits, "");
        println!("Case #{}: {}", case, res);
    }
    Ok(())
}
