use std::io;

/*
Y * S(A) = X * S(B)

1 + 2 + ... + n = n * (n + 1) / 2

Let K be S(B),

Y * (n*(n + 1)/2 - K) = X * K

then S(A) = n * (n + 1) / 2 - K

p = Y * n * (n + 1) / 2

p - Y*K = X*K
p = (X+Y) * K
p / (X + Y) = K, integer

n = 3, X = 1, Y = 2
p = 2 * (3*4/2) = 12

K = 12 / (1 + 2) = 4, possible

S(A) = 3*4/2 - 4 = 2
S(B) = 4
n = 3

1 2 3, collect backwards until we hit S(A)

*/
fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<u64>();
        let n = spec[0];
        let x = spec[1];
        let y = spec[2];

        let p = y * n * (n + 1) / 2;

        let k = if p % (x + y) == 0 {
            // possible
            Some(p / (x + y))
        } else {
            None
        };

        match k {
            Some(val) => {
                println!("Case #{}: {}", case, "POSSIBLE");

                let alan_sum = n * (n + 1) / 2 - val;

                if alan_sum < n {
                    println!("{}", 1);
                    println!("{}", alan_sum);
                } else {
                    // to achieve alan sum we need a few numbers
                    let mut group: Vec<u64> = vec![];
                    let mut next = n;
                    loop {
                        let partial_sum: u64 = group.iter().sum();

                        if partial_sum == alan_sum {
                            break;
                        }

                        if partial_sum + next < alan_sum {
                            group.push(next);
                            next = next - 1;
                        } else {
                            // adding next would overflow
                            group.push(alan_sum - partial_sum)
                        }
                    }
                    println!("{}", group.len());
                    println!("{}", string_vec(&group, " "));
                }
            }

            None => {
                println!("Case #{}: {}", case, "IMPOSSIBLE");
            }
        }
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
