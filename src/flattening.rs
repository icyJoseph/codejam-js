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

fn calc_delta(vec: &[usize]) -> usize {
    use std::collections::HashMap;

    let mut freqs = HashMap::new();

    for i in vec {
        *freqs.entry(i).or_insert(0) += 1;
    }

    let common = freqs
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _)| k)
        .unwrap();

    let max_q = vec
        .iter()
        .filter(|x| x == common)
        .collect::<Vec<&usize>>()
        .len();

    vec.len() - max_q
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<usize>();
        let total = spec[0];
        let target = spec[1];

        let map = parse_vec::<usize>();

        let mut uneven = 0;

        for i in 0..total - 1 {
            let curr = map[i];
            let next = map[i + 1];

            if next != curr {
                uneven += 1;
            }
        }

        let mut rebuild: Option<usize> = None;

        if uneven > target {
            // calculate range
            // calculate best to i
            let mut range = vec![vec![0; total + 1]; total + 1];

            for i in 1..=total {
                for j in i + 1..=total {
                    let view = &map[i - 1..=j - 1];

                    let delta = calc_delta(view);

                    range[i][j] = delta;
                }
            }

            let mut best_fit = vec![vec![0; total + 1]; total + 1];

            // making 1---x with zero changes is equal to range[1][x]
            for x in 1..=total {
                best_fit[x][0] = range[1][x];
            }

            for x in 2..=total {
                for k in 1..=target {
                    let mut acc = vec![];
                    for i in 1..x {
                        let c = best_fit[i][k - 1] + range[i + 1][x];
                        acc.push(c);
                    }

                    best_fit[x][k] = *(acc.iter().min().unwrap());
                }
            }

            rebuild = Some(best_fit[total][target]);
        }

        match rebuild {
            Some(n) => println!("Case #{}: {}", case, n),
            None => println!("Case #{}: {}", case, 0),
        }
    }
    Ok(())
}
