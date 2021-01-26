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

// a more solid way to conclude if it would round up
fn calc_dec(a: usize, b: usize) -> i32 {
    let floor = 100 * a / b;
    let val: f64 = (100.0 * a as f64) / (b as f64);
    ((val - floor as f64) * 100.0) as i32
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

        let mut resp: Vec<usize> = nxt()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let mut diff = people - resp.iter().fold(0, |p, c| p + c);

        resp.sort_by(|a, b| calc_dec(*b, people).cmp(&calc_dec(*a, people)));

        loop {
            for lang in 0..resp.len() {
                let mut dec = calc_dec(resp[lang], people);
                if dec == 0 || dec >= 50 {
                    continue;
                } else {
                    loop {
                        if diff == 0 {
                            break;
                        }

                        resp[lang] = resp[lang] + 1;
                        diff -= 1;
                        dec = calc_dec(resp[lang], people);
                        if dec >= 50 {
                            break;
                        }
                    }
                }
            }

            if diff > 0 {
                resp.push(1);
                diff -= 1;
            } else {
                break;
            }
        }

        let result = resp.iter().fold(0, |prev, &curr| {
            let dec = calc_dec(curr, people);
            if dec >= 50 {
                return prev + (100 * curr / people) + 1;
            } else {
                return prev + (100 * curr / people);
            }
        });

        println!("Case #{}: {}", case, result);
    }
    Ok(())
}
