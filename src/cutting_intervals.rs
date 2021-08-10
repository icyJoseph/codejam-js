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

#[allow(dead_code)]
fn string_vec<T: std::string::ToString>(vec: &Vec<T>, separator: &str) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(separator)
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<u64>();

        let mut interval_count = spec[0];
        let max_cuts = spec[1];

        use std::collections::HashMap;

        let mut freq: HashMap<u64, u64> = HashMap::new();

        for _ in 0..interval_count {
            let bounds = parse_vec::<u64>();
            let lower = bounds[0] + 1;
            let upper = bounds[1];

            for i in lower..upper {
                *freq.entry(i).or_insert(0) += 1;
            }
        }

        let mut counts: Vec<u64> = freq.iter().map(|(_, v)| *v).collect();

        counts.sort_by(|a, b| a.cmp(b).reverse());

        let mut cuts = 0;

        for q in counts {
            if cuts == max_cuts {
                break;
            }

            interval_count = interval_count - q + 2 * q;

            cuts += 1;
        }

        println!("Case #{}: {}", case, interval_count);
    }
    Ok(())
}
