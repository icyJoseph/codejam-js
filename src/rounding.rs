use std::io;

type Res<T> = Result<T, Box<dyn std::error::Error>>;

fn nxt() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        _ => panic!("Error reading line"),
    }
}

fn ptc<T: std::str::FromStr>() -> T {
    match nxt().trim().parse::<T>() {
        Ok(n) => n,
        _ => panic!("Error parsing"),
    }
}

fn does_round_up(a: usize, b: usize) -> bool {
    2 * (100 * a % b) >= b
}

fn distance(a: usize, b: usize) -> usize {
    let mut delta = 0;

    if 100 * a % b == 0 {
        return delta;
    }

    while !does_round_up(a + delta, b) {
        delta += 1;
    }

    delta
}

fn find_minr(n: usize) -> usize {
    let mut min = 1;

    if 100 * min % n == 0 {
        return min;
    }

    while !does_round_up(min, n) {
        min += 1;
    }

    min
}

fn round_up(a: usize, b: usize) -> usize {
    if does_round_up(a, b) {
        return 100 * a / b + 1;
    }
    return 100 * a / b;
}

fn main() -> Res<()> {
    let n = ptc::<i32>();

    for case in 1..=n {
        let spec: Vec<usize> = nxt()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let people = spec[0];

        let mut resp: Vec<(usize, usize)> = nxt()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .map(|x| (x, distance(x, people)))
            .collect();

        let mut diff = people - resp.iter().fold(0, |p, c| p + c.0);

        resp.sort_by(|a, b| a.1.cmp(&b.1));

        let mut total = 0;

        for (qty, dist) in resp {
            if diff >= dist {
                let delta = round_up(qty + dist, people);

                total += delta;
                diff -= dist;
            } else {
                let delta = round_up(qty, people);

                total += delta;
            }
        }

        let minr = find_minr(people);

        let pad = diff / minr;
        let leftover = diff % minr;

        let delta = round_up(minr, people);
        let leftover_delta = round_up(leftover, people);

        total += pad * delta;
        total += leftover_delta;

        println!("Case #{}: {}", case, total);
    }
    Ok(())
}
