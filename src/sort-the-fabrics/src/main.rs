use std::io;

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let total = parse_num::<usize>();

        let mut ada_sort: Vec<(String, usize, usize)> = vec![];
        let mut charles_sort: Vec<(String, usize, usize)> = vec![];

        for _ in 0..total {
            let spec_raw = nxt();
            let spec = spec_raw.split_whitespace().collect::<Vec<_>>();

            let comp = spec[1..]
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            ada_sort.push((spec[0].to_string(), comp[0], comp[1]));
            charles_sort.push((spec[0].to_string(), comp[0], comp[1]));
        }

        ada_sort.sort_by(|a, b| {
            if a.0 == b.0 {
                return a.2.cmp(&b.2);
            }

            a.0.cmp(&b.0)
        });

        charles_sort.sort_by(|a, b| {
            if a.1 == b.1 {
                return a.2.cmp(&b.2);
            }

            a.1.cmp(&b.1)
        });


        let mut answer = 0;

        for pos in 0..total {
            if ada_sort[pos].2 == charles_sort[pos].2 {
                answer += 1;
            }
        }

        println!("Case #{}: {}", case, answer);
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
fn string_vec<T: std::string::ToString>(vec: &[T], separator: &str) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(separator)
}
