use std::io;

/*
N regions
A1, A2, A3... number of participants from each region

M categories

Ai, Aj -> Mn

S(Mn) = median(Ai, Aj,...)

S(Campaign) = S(M1) + S(M2) + ...

3 regions, A1 = 5, A2 = 8, A3 = 9

2 categories

A2, A3 -> M1
A1 -> M2

S(M1) = median(8,9) = 8.5
S(M2) = 5

S(Campaign) = 13.5

find the maximum possible value of the
success metric that can be obtained by
assigning participants in regions to
the categories.

N = 3, C = 2

A1 11, A2 24, A3 10

S = median(C1) + median(C2)

Sort categories desc

10 9 8 7 6 5 4 3 2 1

For example 4 categories

[10, 9, 8, 7]

We have 6 5 4 3 2 1 leftover

[10, 9, 8, 7 6 5 4 3 2 1]
10+9+8+4 = 31

The highest sum is by keeping the other numbers and reduce the right most

*/

fn median(list: &Vec<usize>) -> f64 {
    let len = list.len();

    if len % 2 == 0 {
        let left = list[len / 2 - 1];
        let right = list[len / 2];

        return (left + right) as f64 / 2.0;
    }

    return (list[len / 2] as f64) / 1.0;
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<usize>();
        let total_regions = spec[0];
        let total_cats = spec[1];

        let mut regions = parse_vec::<usize>();

        regions.sort_by(|a, b| a.cmp(&b).reverse());

        let mut assignments: Vec<Vec<usize>> = vec![vec![]; total_cats];

        for index in 0..total_cats {
            assignments[index].push(regions[index]);
        }

        for leftover in total_cats..total_regions {
            assignments[total_cats - 1].push(regions[leftover])
        }

        let mut answer = 0.0;

        for list in assignments {
            answer += median(&list);
        }

        println!("Case #{}: {:.1}", case, answer);
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
