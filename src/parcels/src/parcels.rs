use std::collections::VecDeque;
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

fn calc_adj(r: usize, c: usize) -> Vec<Vec<usize>> {
    let norm = |i: usize, j: usize| c * i + j;

    let mut adj = vec![vec![]; r * c];

    for i in 0..r {
        for j in 0..c {
            let index = norm(i, j);
            if i > 0 {
                adj[index].push(norm(i - 1, j));
            }
            if j + 1 < c {
                adj[index].push(norm(i, j + 1));
            }
            if i + 1 < r {
                adj[index].push(norm(i + 1, j));
            }
            if j > 0 {
                adj[index].push(norm(i, j - 1));
            }
        }
    }

    adj
}

fn bfs(
    adj: &Vec<Vec<usize>>,
    distances: &mut Vec<usize>,
    r: usize,
    c: usize,
    sources: &Vec<(usize, usize)>,
) {
    let mut visited = vec![false; r * c];

    let mut q = VecDeque::new();

    for (x, y) in sources {
        let node = x * c + y;
        visited[node] = true;
        q.push_back(node);
    }

    loop {
        let next = q.pop_front();

        match next {
            None => {
                break;
            }
            Some(elem) => {
                for &node in adj[elem].iter() {
                    if visited[node] {
                        continue;
                    }

                    visited[node] = true;

                    distances[node] = distances[elem] + 1;
                    q.push_back(node);
                }
            }
        }
    }
}

fn calc_distances(
    adj: &Vec<Vec<usize>>,
    r: usize,
    c: usize,
    offices: &Vec<(usize, usize)>,
) -> Vec<usize> {
    let mut distances = vec![0; r * c];

    if offices.len() == 0 {
        return vec![r + c - 2];
    }

    bfs(&adj, &mut distances, r, c, &offices);

    distances
}

fn bin_search(
    adj: &Vec<Vec<usize>>,
    distances: Vec<usize>,
    offices: &mut Vec<(usize, usize)>,
    blanks: &Vec<(usize, usize)>,
    r: usize,
    c: usize,
    look_up: &Vec<(usize, usize, usize)>,
) -> usize {
    if distances.len() == 2 {
        let distance = distances[0];
        let targets = look_up
            .into_iter()
            .filter(|(_, _, dist)| *dist > distance)
            .map(|(x, y, _)| return (*x as isize, *y as isize))
            .collect::<Vec<(isize, isize)>>();

        for blank in blanks {
            let possible = targets.iter().all(|(x, y)| {
                return (x - blank.0 as isize).abs() + (y - blank.1 as isize).abs()
                    <= distance as isize;
            });

            if possible {
                return distances[0];
            }
        }

        return distances[1];
    }

    if distances.len() == 1 {
        return distances[0];
    }

    let lower = 0;
    let upper = distances.len();
    let middle = (lower + upper) / 2;
    let distance = distances[middle];

    let targets = look_up
        .into_iter()
        .filter(|(_, _, dist)| *dist > distance)
        .map(|(x, y, _)| return (*x as isize, *y as isize))
        .collect::<Vec<(isize, isize)>>();

    for blank in blanks {
        let possible = targets.iter().all(|(x, y)| {
            return (x - blank.0 as isize).abs() + (y - blank.1 as isize).abs()
                <= distance as isize;
        });

        if possible {
            return bin_search(
                &adj,
                distances[0..=middle].to_vec(),
                offices,
                blanks,
                r,
                c,
                &look_up,
            );
        }
    }

    return bin_search(
        &adj,
        distances[middle..].to_vec(),
        offices,
        blanks,
        r,
        c,
        &look_up,
    );
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let rc = parse_vec::<usize>();

        let r = rc[0];
        let c = rc[1];

        let mut offices = vec![];
        let mut blanks = vec![];

        for i in 0..r {
            let next = nxt();

            for (j, c) in next.trim().chars().enumerate() {
                if c == '1' {
                    offices.push((i, j));
                } else {
                    blanks.push((i, j));
                }
            }
        }

        if blanks.len() == 0 {
            println!("Case #{}: {}", case, 0);
            continue;
        }

        if r == 1 && c == 1 {
            println!("Case #{}: {}", case, 0);
            continue;
        }

        let adj = calc_adj(r, c);

        let mut distances = calc_distances(&adj, r, c, &offices);

        let mut look_up = vec![];

        for (node, distance) in distances.iter().enumerate() {
            let x = node / c;
            let y = node % c;
            look_up.push((x, y, *distance));
        }

        distances.sort_by(|a, b| a.cmp(&b));
        distances.dedup();

        let result = bin_search(&adj, distances, &mut offices, &blanks, r, c, &look_up);

        println!("Case #{}: {}", case, result);
    }
    Ok(())
}
