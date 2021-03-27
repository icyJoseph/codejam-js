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

fn reverse_section(list: &Vec<usize>, from: usize, to: usize) -> Vec<usize> {
    let mut answer: Vec<usize> = vec![];

    for index in 0..from {
        answer.push(list[index]);
    }

    for index in (from..=to).rev() {
        answer.push(list[index]);
    }

    for index in to + 1..list.len() {
        answer.push(list[index]);
    }

    return answer;
}

fn rev_cost(n: usize, target: usize) -> Option<Vec<usize>> {
    if target < n - 1 {
        return None;
    }
    if target > n * (n + 1) / 2 - 1 {
        return None;
    }

    // special cases
    if target == n - 1 {
        let answer = (1..=n).collect();
        return Some(answer);
    }

    if target == n {
        let mut answer: Vec<usize> = (1..=n).collect();

        let last_index = answer.len() - 1;
        answer.swap(last_index, last_index - 1);

        return Some(answer);
    }

    // build the vector
    let mut comps: Vec<usize> = (2..=n).collect();
    let mut curr = 0;

    loop {
        let sum = comps.iter().sum::<usize>();

        if sum == target {
            break;
        }

        if comps[curr] > 1 {
            comps[curr] = comps[curr] - 1;
            continue;
        }
        curr += 1;
    }

    let mut answer: Vec<usize> = (1..=n).collect();
    let mut pointer = n - 2;

    for inst in comps.iter() {
        answer = reverse_section(&answer, pointer, pointer + inst - 1);
        if pointer > 0 {
            pointer -= 1;
        }
    }

    Some(answer)
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<usize>();

        let upper = spec[0];
        let cost = spec[1];

        if upper <= 7 {
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
        } else {
            match rev_cost(upper, cost) {
                Some(l) => {
                    println!("Case #{}: {}", case, print_ls(l));
                }
                None => {
                    println!("Case #{}: {}", case, "IMPOSSIBLE");
                }
            }
        }
    }
    Ok(())
}
