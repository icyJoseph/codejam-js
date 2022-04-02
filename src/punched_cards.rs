use std::io;

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let dimensions = parse_vec::<usize>();

        let rows = dimensions[0];
        let cols = dimensions[1];

        let cols_pixels = 2 * cols + 1;
        let rows_pixels = 2 * rows + 1;

        let mut grid = String::new();

        for r in 0..rows_pixels {
            for c in 0..cols_pixels {
                if r % 2 == 0 {
                    if r == 0 && c < 2 {
                        // top left ..
                        grid = format!("{}.", grid);
                    } else if c % 2 == 0 {
                        // +
                        grid = format!("{}+", grid);
                    } else {
                        // -
                        grid = format!("{}-", grid);
                    }
                } else {
                    // actual content
                    if c == 0 && r == 1 {
                        grid = format!("{}.", grid);
                    } else if c % 2 == 1 {
                        grid = format!("{}.", grid);
                    } else {
                        grid = format!("{}|", grid);
                    }
                }
            }

            grid = format!("{}\n", grid);
        }

        println!("Case #{}:\n{}", case, grid);
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
