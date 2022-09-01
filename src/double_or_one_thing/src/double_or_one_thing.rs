use std::io;

fn main() -> Res<()> {
    let n = parse_num::<u32>();

    for case in 1..=n {
        let input = nxt();
        let word = input.trim();

        let mut result = String::new();

        let chars = word.chars().collect::<Vec<_>>();

        for (i, ch) in chars.iter().enumerate() {
            let rest = chars[i + 1..].to_vec();
            let ext = chars[i..].to_vec();

            let rest_tail = format!("{}", string_vec(&rest, ""));
            let ext_tail = format!("{}", string_vec(&ext, ""));

            if rest_tail.lt(&ext_tail) {
                result = format!("{}{}", result, ch);
            } else {
                result = format!("{}{}{}", result, ch, ch);
            }
        }

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
