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

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<usize>();

        let pool = spec[1];

        let mut sold = parse_vec::<u64>();

        sold.sort_by(|a, b| a.cmp(&b));

        sold.dedup();

        let mut probability = 0.0f64;
        if sold.len() < pool {
            let mut nodes: Vec<(u64, u64)> = vec![];

            for (i, &entry) in sold.iter().enumerate() {
                if i == sold.len() - 1 {
                    nodes.push((entry, (pool as u64) - entry));
                } else {
                    nodes.push((entry, (sold[i + 1] - entry) / 2));
                    if (sold[i + 1] - entry) / 2 > 1 {
                        nodes.push((sold[i + 1], (sold[i + 1] - entry) / 2));
                    }
                }
            }

            nodes.sort_by(|a, b| a.1.cmp(&b.1).reverse());

            if nodes.len() == 0 {
                probability = 0.0;
            } else if nodes.len() == 1 {
                probability = (nodes[0].1 as f64) / (pool as f64)
            } else {
                probability = (nodes[0].1 + nodes[1].1) as f64 / (pool as f64)
            }
        }

        println!("Case #{}: {:.6}", case, probability);
    }
    Ok(())
}
