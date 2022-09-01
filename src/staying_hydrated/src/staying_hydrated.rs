use std::io;

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let total = parse_num::<isize>();

        let mut objects = vec![];

        for _ in 0..total {
            objects.push(parse_vec::<isize>());
        }

        let mut x = vec![];
        let mut y = vec![];

        for obj in objects.iter() {
            let x0 = obj[0];
            let y0 = obj[1];

            let x1 = obj[2];
            let y1 = obj[3];

            x.push(x0);
            x.push(x1);
            y.push(y0);
            y.push(y1);
        }

        x.sort();
        x.dedup();

        y.sort();
        y.dedup();

        let x_coord = || {
            for target in x {
                let ahead = objects
                    .iter()
                    .filter(|coords| {
                        return target < coords[0];
                    })
                    .count();

                let behind = objects
                    .iter()
                    .filter(|coords| {
                        return coords[2] <= target;
                    })
                    .count();

                if behind >= ahead {
                    return Some(target);
                }
            }

            None
        };

        let y_coord = || {
            for target in y {
                let ahead = objects
                    .iter()
                    .filter(|coords| {
                        return target < coords[1];
                    })
                    .count();

                let behind = objects
                    .iter()
                    .filter(|coords| {
                        return coords[3] <= target;
                    })
                    .count();

                if behind >= ahead {
                    return Some(target);
                }
            }

            None
        };

        match (x_coord(), y_coord()) {
            (Some(_x), Some(_y)) => {
                println!("Case #{}: {} {}", case, _x, _y);
            }
            _ => panic!("Failed to find coordinates"),
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
