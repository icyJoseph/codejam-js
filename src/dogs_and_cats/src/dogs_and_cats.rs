use std::io;

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<usize>();

        let mut dogs_portions = spec[1];
        let mut cats_portions = spec[2];
        let extra_portions = spec[3];

        let raw_q = nxt();

        let q = raw_q.trim().chars().collect::<Vec<_>>();

        let total_dogs = q.iter().filter(|&c| *c == 'D').count();

        let mut fed_dogs = 0;

        for next in q {
            match next {
                'C' => {
                    if cats_portions == 0 {
                        break;
                    }

                    cats_portions = cats_portions - 1;
                }
                'D' => {
                    if dogs_portions == 0 {
                        break;
                    }

                    dogs_portions = dogs_portions - 1;
                    fed_dogs = fed_dogs + 1;
                    cats_portions = cats_portions + extra_portions;
                }
                _ => panic!("Unexpected animal"),
            }
        }

        println!(
            "Case #{}: {}",
            case,
            if fed_dogs == total_dogs { "YES" } else { "NO" }
        );
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
