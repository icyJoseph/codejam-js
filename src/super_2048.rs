use std::io;

fn squeeze(row: &mut Vec<usize>) {
    let size = row.len();

    for i in 0..size {
        if row[i] == 0 {
            row.remove(i);
            row.insert(0, 0);
        }
    }
}

fn row_collapse(row: &mut Vec<usize>, size: usize) {
    squeeze(row);

    for i in (1..size).rev() {
        if row[i] == 0 {
            continue;
        }
        if row[i] == row[i - 1] {
            row[i] = 2 * row[i];
            row[i - 1] = 0;
        }
    }

    squeeze(row);
}

fn collapse(grid: &mut Vec<Vec<usize>>, dir: &str, size: usize) {
    match dir {
        "up" => {
            for col in 0..size {
                let mut row = grid.iter().rev().map(|r| r[col]).collect::<Vec<_>>();
                row_collapse(&mut row, size);

                for r in 0..size {
                    grid[size - 1 - r][col] = row[r]
                }
            }
        }
        "down" => {
            for col in 0..size {
                let mut row = grid.iter().map(|r| r[col]).collect::<Vec<_>>();
                row_collapse(&mut row, size);

                for r in 0..size {
                    grid[r][col] = row[r];
                }
            }
        }
        "right" => {
            for row in 0..size {
                row_collapse(&mut grid[row], size);
            }
        }
        "left" => {
            for row in 0..size {
                let mut rev_row = grid[row].iter().map(|&v| v).rev().collect::<Vec<_>>();
                row_collapse(&mut rev_row, size);

                for r in 0..size {
                    grid[row][size - 1 - r] = rev_row[r];
                }
            }
        }
        _ => {}
    }
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let raw_spec = nxt();
        let spec = raw_spec.trim().split_whitespace().collect::<Vec<_>>();

        let size = spec[0].parse::<usize>().unwrap();
        let direction = spec[1];

        let mut grid = vec![];

        for _ in 0..size {
            let row = parse_vec::<usize>();
            grid.push(row);
        }

        collapse(&mut grid, direction, size);

        let mut result = format!("");

        for row in grid {
            result = format!("{}\n{}", result, string_vec(&row, " "));
        }

        println!("Case #{}: {}", case, result);
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
