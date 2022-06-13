use std::io;

fn is_valid(vec: &Vec<usize>, size: usize) -> bool {
    use std::collections::HashSet;
    let mut set = HashSet::new();

    for &val in vec {
        if val < 1 || val > size * size {
            return false;
        }

        set.insert(val);
    }

    return set.len() == vec.len();
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let size = parse_num::<usize>();

        let mut rows = vec![];

        for _ in 0..size * size {
            let row = parse_vec::<usize>();
            rows.push(row);
        }

        let mut has_error = None;

        for r in 0..size * size {
            let valid_row = is_valid(&rows[r], size);

            if !valid_row {
                has_error = Some(());
                break;
            }

            for c in 0..size * size {
                let col = rows.iter().map(|row| row[c]).collect::<Vec<usize>>();
                let valid_col = is_valid(&col, size);

                if !valid_col {
                    has_error = Some(());
                    break;
                }

                if r % size == 0 && c % size == 0 {
                    // corner of a sub grid
                    let mut flat_sub_grid = vec![];

                    for i in r..r + size {
                        for j in c..c + size {
                            flat_sub_grid.push(rows[i][j]);
                        }
                    }

                    if !is_valid(&flat_sub_grid, size) {
                        has_error = Some(());
                        break;
                    }
                }
            }

            match has_error {
                None => { /* continue */ }
                _ => break,
            }
        }

        println!(
            "Case #{}: {}",
            case,
            match has_error {
                None => "Yes",
                _ => "No",
            }
        )
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
