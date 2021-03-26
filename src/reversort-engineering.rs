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

fn calc_cost(input: &Vec<usize>) -> usize {
    let mut cost = 0;

    let mut list = input.clone();

    let mut sorted = list.clone();

    sorted.sort();

    for (i, entry) in sorted.iter().enumerate() {
        let pos = list.iter().position(|x| x == entry).unwrap();

        if i == list.len() - 1 {
            break;
        }

        let rev: Vec<usize> = list[i..=pos].iter().rev().map(|&x| x).collect();
        cost += (pos - i) + 1;

        for (x, &el) in rev.iter().enumerate() {
            list[x + i] = el;
        }
    }
    cost
}

fn print_ls(ls: Vec<usize>) -> String {
    ls.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn search(
    permutation: &mut Vec<usize>,
    chosen: &mut Vec<bool>,
    n: usize,
    cost: usize,
) -> Option<Vec<usize>> {
    if permutation.len() == n {
        if calc_cost(permutation) == cost {
            return Some(permutation.to_vec());
        } else {
            return None;
        }
    } else {
        for i in 0..n {
            if chosen[i] {
                continue;
            }

            chosen[i] = true;
            permutation.push(i + 1);
            let res = search(permutation, chosen, n, cost);
            if res.is_none() {
                chosen[i] = false;
                permutation.pop();
            } else {
                return res;
            }
        }
    }
    return None;
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<usize>();

        let upper = spec[0];
        let cost = spec[1];

        let list: Vec<usize> = (1..=upper).collect();

        let mut permutation: Vec<usize> = vec![];
        let mut chosen = vec![false; upper];

        match search(&mut permutation, &mut chosen, upper, cost) {
            Some(l) => {
                println!("Case #{}: {}", case, print_ls(l));
            }
            None => {
                println!("Case #{}: {}", case, "IMPOSSIBLE");
            }
        }
    }
    Ok(())
}
