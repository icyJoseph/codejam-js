use std::io;

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let _ = parse_num::<usize>();

        let days = parse_vec::<usize>();

        let mut peak: Option<usize> = None;

        let mut record_days = 0;

        for i in 0..days.len() {
            let first_condition = i == 0 || {
                match peak {
                    Some(m) if days[i] <= m => false,
                    _ => true,
                }
            };

            let second_condition = i == days.len() - 1 || { days[i] > days[i + 1] };

            if first_condition && second_condition {
                record_days += 1;
            }

            match peak {
                None => peak = Some(days[i]),
                Some(m) if m < days[i] => {
                    peak = Some(days[i]);
                }
                _ => {}
            }
        }

        println!("Case #{}: {}", case, record_days);
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
