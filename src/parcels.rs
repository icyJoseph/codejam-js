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

fn m_d(a: (usize, usize), b: (usize, usize)) -> usize {
    use std::cmp::{max, min};

    (max(a.0, b.0) - min(a.0, b.0)) + (max(a.1, b.1) - min(a.1, b.1))
}

fn calc_distances(r: usize, c: usize, offices: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut distances = vec![];

    for i in 0..r {
        for j in 0..c {
            let mut distance = None;

            for office in offices.iter() {
                let d = m_d((i, j), (office.0, office.1));

                match distance {
                    None => {
                        distance = Some(d);
                    }
                    Some(v) if v > d => {
                        distance = Some(d);
                    }
                    _ => {}
                }
            }

            match distance {
                None => distances.push(r + c),
                Some(v) => distances.push(v),
            }
        }
    }

    distances
}

fn search(
    distances: Vec<usize>,
    offices: &mut Vec<(usize, usize)>,
    blanks: &Vec<(usize, usize)>,
    r: usize,
    c: usize,
) -> usize {
    // take the middle and check if possible
    if distances.len() == 2 {
        for distance in distances.iter() {
            for blank in blanks.iter() {
                offices.push((blank.0, blank.1));

                let new_distances = calc_distances(r, c, &offices);
                let candidate = new_distances.into_iter().max().unwrap();

                offices.pop();

                if candidate == *distance {
                    return candidate;
                }
            }
        }
    }

    if distances.len() == 1 {
        return distances[0];
    }

    let lower = 0;
    let upper = distances.len();
    let middle = (lower + upper) / 2;
    let pivot = distances[middle];

    for blank in blanks.iter() {
        offices.push((blank.0, blank.1));

        let new_distances = calc_distances(r, c, &offices);
        let candidate = new_distances.into_iter().max().unwrap();

        offices.pop();

        if candidate == pivot {
            // possible branch
            // try to do better
            return search(distances[0..=middle].to_vec(), offices, blanks, r, c);
        }
    }

    // not possible
    // try to do worse
    return search(distances[middle..].to_vec(), offices, blanks, r, c);
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let rc = parse_vec::<usize>();

        let r = rc[0];
        let c = rc[1];

        let mut grid = vec![];

        let mut offices = vec![];
        let mut blanks = vec![];

        for i in 0..r {
            let next = nxt();
            let row: Vec<u32> = next
                .trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect();

            for j in 0..c {
                if row[j] == 1 {
                    offices.push((i, j));
                } else {
                    blanks.push((i, j));
                }
            }
            grid.push(row);
        }

        if r == 1 && c == 1 {
            println!("Case #{}: {}", case, 0);
            continue;
        }

        let mut distances = calc_distances(r, c, &offices);
        distances.sort_by(|a, b| a.cmp(&b));

        let result = search(distances, &mut offices, &blanks, r, c);

        println!("Case #{}: {}", case, result);
    }
    Ok(())
}
