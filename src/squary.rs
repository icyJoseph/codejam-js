use std::io;

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        /*
        list of integers not necessarily squary

        can we add 1 and at most K such that result is squary

        each element must be -10^18 - 10^18 inclusive

        no distinction

        (a + b + c + d)^2 = a^2 + b^2 + c^2 + d^2 => squary

        (a + b + c + d) * (a + b + c + d)

        a*a + a * (b + c + d) + b*b + b*(a + c + d)...

        a * (b + c + d) + b * (a + c + d) + c (a + b + d) +
        d (a + b + c) = 0

        ===

        (a + b + c) = a*a + b*b + c*c
        a*a + a(b+c) + b*b + b(a+c) + c*c + c(a+b) = S

        with d

        S + d*d + d(a+b+c) + a*d + b*d + c*d = 0

        a*a + a(b+c) + b*b + b(a+c) + c*c + c(a+b) +
        d*d + d(a+b+c) + a*d + b*d + c*d

        a*a + b*b + c*c + a(b+c) + b(a+c) + c(a+b) +
        d*d + d(a+b+c) + a*d + b*d + c*d

        a(b+c) + b(a+c) + c(a+b) +
        d(a+b+c) + a*d + b*d + c*d = 0

        a(b+c) + b(a+c) + c(a+b) +
        2*d(a+b+c) = 0

        a(b+c) + b(a+c)+ c(a+b) / (2*(a+b+c)) = - d

        -2 6 0

        -2(6) + 6(-2) = -24

        -24 / (2*(4)) = -24 / 8 = -3 => d = 3

        ====


        (a + b + c)^2 = a*a + b*b + c*c
        a*a + a(b+c) + b*b + b(a+c) + c*c + c(a+b) = S

        d e f...k

        */

        let spec = parse_vec::<i64>();

        let total = spec[0];

        let max_k = spec[1];

        let numbers = parse_vec::<i64>();

        let init_sum: i64 = numbers.iter().sum();

        if max_k >= 2 {
            /**
            (a + b + c + 1 - a - b + q) = a*a + b*b + c*c + p*p + q*q

            where, p = 1 - a - b

            then, a*a + b*b + c*c + p*p + q*q = (1+q)*(1+q)
            sum of squares + p*p = 1 + 2*q
            (sum of squares + p*p - 1) / 2 = q;

            */
            let p = 1 - init_sum;
            let q: i64 = (numbers.iter().map(|x| x * x).sum::<i64>() + p * p - 1) / 2;

            println!("Case #{}: {} {}", case, p, q);
        } else {
            if init_sum == 0 {
                if numbers.iter().all(|&x| x == 0) {
                    println!("Case #{}: {}", case, 0);
                } else {
                    println!("Case #{}: {}", case, "IMPOSSIBLE");
                }
            } else {
                let acc_sum = {
                    let mut total = 0;

                    for n in numbers.iter() {
                        total = total + n * (init_sum - n);
                    }

                    total
                };

                if acc_sum % 2 == 0 && acc_sum % init_sum == 0 {
                    let extra = -acc_sum / (2 * (init_sum));

                    println!("Case #{}: {}", case, extra);
                } else {
                    println!("Case #{}: {}", case, "IMPOSSIBLE");
                }
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
