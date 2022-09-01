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

fn main() -> Res<()> {
    let n = parse_num::<u32>();

    for case in 1..=n {
        let spec = parse_vec::<usize>();

        let total = spec[0];
        let first = spec[1];
        let second = spec[2];
        let common = spec[3];

        let f_p = first - common;
        let s_p = second - common;

        if (f_p + s_p + common) > total {
            println!("Case #{}: {}", case, "IMPOSSIBLE");
        } else {
            let mut fill = total - (f_p + s_p + common);

            let vec_answer = {
                let mut v = vec![];

                for _ in 0..f_p {
                    v.push(total - 1);
                }

                if v.len() > 0 {
                    for _ in 0..fill {
                        v.push(1);
                    }
                    fill = 0;
                }

                for _ in 0..common {
                    v.push(total);
                }

                if s_p > 0 {
                    for _ in 0..fill {
                        v.push(1);
                    }
                    fill = 0;
                }

                if common > 1 {
                    if fill > 0 {
                        let end = v.pop().unwrap();
                        for _ in 0..fill {
                            v.push(1);
                        }
                        v.push(end);
                        fill = 0;
                    }
                }

                for _ in 0..s_p {
                    v.push(total - 1);
                }

                v
            };

            if fill > 0 {
                println!("Case #{}: {}", case, "IMPOSSIBLE");
            } else {
                let answer = vec_answer
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");

                println!("Case #{}: {}", case, answer);
            }
        }
    }

    Ok(())
}
