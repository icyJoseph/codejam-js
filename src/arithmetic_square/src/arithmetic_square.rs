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

fn is_arithmetic_prog(vec: &Vec<i64>) -> bool {
    2 * vec[1] == vec[2] + vec[0]
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let top = parse_vec::<i64>();
        let middle = parse_vec::<i64>();
        let bottom = parse_vec::<i64>();

        let left = vec![top[0], middle[0], bottom[0]];
        let right = vec![top[2], middle[1], bottom[2]];

        let mut count = 0;

        if is_arithmetic_prog(&left) {
            count += 1;
        }
        if is_arithmetic_prog(&right) {
            count += 1;
        }
        if is_arithmetic_prog(&top) {
            count += 1;
        }
        if is_arithmetic_prog(&bottom) {
            count += 1;
        }

        use std::collections::HashMap;

        let mut map: HashMap<i64, u32> = HashMap::new();

        let centers: Vec<i64> = vec![
            top[0] + bottom[2],
            top[2] + bottom[0],
            middle[0] + middle[1],
            top[1] + bottom[1],
        ]
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|&x| x / 2)
        .collect();

        for c in centers {
            *map.entry(c).or_insert(0) += 1;
        }

        let max = match map.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(_, v)| v) {
            Some(&n) => n,
            None => 0,
        };

        println!("Case #{}: {}", case, count + max);
    }
    Ok(())
}
