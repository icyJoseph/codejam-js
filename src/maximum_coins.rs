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
        let size = parse_num::<usize>();
        let mut grid = vec![];

        for _ in 0..size {
            let row = parse_vec::<u64>();
            grid.push(row);
        }

        let mut max = 0;

        for i in 0..size {
            if i == 0 {
                let mut total = 0;
                for j in 0..size {
                    total += grid[j][j];
                }
                if total > max {
                    max = total;
                }
            } else {
                let mut left = 0;
                let mut right = 0;

                let mut x = 0;
                for j in i..size {
                    left += grid[x][j];
                    right += grid[j][x];
                    x += 1;
                }

                if left > max {
                    max = left;
                }
                if right > max {
                    max = right;
                }
            }
        }
        println!("Case #{}: {}", case, max);
    }

    Ok(())
}
