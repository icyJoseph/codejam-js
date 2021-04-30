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

fn string_vec<T: std::string::ToString>(vec: &Vec<T>) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let _total = parse_num::<usize>();
        let cits = parse_vec::<usize>();

        let mut history = vec![];
        let mut h_index = 0;
        let mut q = vec![0];

        for c in cits {
            if q.len() <= c {
                for _ in q.len()..=c {
                    q.push(q.len());
                }
            }

            if c > h_index {
                for i in h_index..=c {
                    if q[i] > 0 {
                        q[i] -= 1;
                        if q[i] == 0 {
                            h_index = i;
                        }
                    }
                }
            }
            history.push(h_index);
        }

        println!("Case #{}: {}", case, string_vec(&history));
    }
    Ok(())
}
