use std::io;

/**
root 1/1
1/2 2/1

for any p/q -> [p / (p+q), (p+q) / q]

Left turn
a/b -> p = a, and b = p + q, q = b - p
p = a, q = b - a, b > a

Right turn
a/b -> a = p + q, q = b,
p = a - b, q = b, a > b

type 1 problem: n = 2
p = 1 q = 2

type 2 problem: p, q given
find n

p = 1, q = 2 -> n = 2

for p/q -> two possible parents a/b c/d

from the left
a/(a+b) = p/q
p = a, q = a + b

a = p, b = q - p

from the right
(a+b)/b = p/q
p = a + b, q = b

a = p - q, b = q

For type 1:

[1, 2, 3, 4, 5, 6, 7,...]

1 -> (2,3)

2 -> (4,5)

3 -> (6,7)

n -> (2*n, 2*n + 1)

given m -> parent = m / 2, if m % 2 == 0 from the left, otherwise from the right

walk up to 1 taking note of left right turns, and then calculate the final p / q walking
downwards

This would be O(ln m)

For type 2:

calculate the two possible parents, keep only positive branches

for p = 3 q = 2

a = 1, b = 2 (parent from the right 3/2 is right child of 1/2)

c = 3, d = 2 - 3 (x negative)

*/

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<u64>();

        let kind = spec[0];

        match kind {
            1 => {
                let target = spec[1];

                let mut path = vec![];
                let mut parent = target;

                loop {
                    if parent == 1 {
                        break;
                    }
                    let dir = parent % 2;
                    path.push(dir);
                    parent = parent / 2;
                }

                let mut r_p = 1_u64;
                let mut r_q = 1_u64;

                for turn in path.iter().rev() {
                    match turn {
                        0 => {
                            // left turn
                            r_p = r_p;
                            r_q = r_q + r_p;
                        }
                        1 => {
                            r_p = r_p + r_q;
                            r_q = r_q;
                        }
                        _ => {}
                    }
                }

                println!("Case #{}: {} {}", case, r_p, r_q);
            }
            2 => {
                let p = spec[1];
                let q = spec[2];

                let mut path = vec![];

                let mut a = p;
                let mut b = q;

                loop {
                    if a == b {
                        break;
                    }

                    if a > b {
                        // right path child
                        a = a - b;
                        b = b;
                        path.push(1);
                    } else {
                        a = a;
                        b = b - a;
                        path.push(0);
                    }
                }

                let mut index = 1_u64;

                for turn in path.iter().rev() {
                    match turn {
                        0 => {
                            // left turn
                            index = index * 2;
                        }
                        1 => {
                            index = index * 2 + 1;
                        }
                        _ => {}
                    }
                }

                println!("Case #{}: {}", case, index)
            }
            _ => {}
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
