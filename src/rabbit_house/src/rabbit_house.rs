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

fn calc_adj(grid: &Vec<Vec<u64>>, x: i64, y: i64) -> Vec<u64> {
    let steps = vec![[1, 0], [-1, 0], [0, 1], [0, -1]];
    let mut adj = vec![];

    for step in steps {
        let dx = step[0];
        let dy = step[1];

        match (x).checked_add(dx) {
            Some(_x) => match grid.get(_x as usize) {
                Some(r) => match (y).checked_add(dy) {
                    Some(_y) => match r.get(_y as usize) {
                        Some(c) => adj.push(*c),
                        None => {}
                    },
                    None => {}
                },
                None => {}
            },
            None => {}
        };
    }

    return adj;
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<usize>();

        let r = spec[0];
        let c = spec[1];

        let mut grid = vec![];

        for _ in 0..r {
            let row = parse_vec::<u64>();

            grid.push(row);
        }

        let mut boxes = 0;

        loop {
            let mut dirty = false;

            for i in 0..r {
                for j in 0..c {
                    let adj = calc_adj(&grid, i as i64, j as i64);

                    let max = adj.iter().max();

                    match max {
                        Some(&m) => {
                            if grid[i][j] < m {
                                let add = m - grid[i][j] - 1;
                                dirty = if add > 0 { true } else { dirty };
                                boxes = boxes + add;
                                grid[i][j] = grid[i][j] + add;
                            }
                        }
                        None => {}
                    }
                }
            }

            if !dirty {
                break;
            }
        }

        println!("Case #{}: {}", case, boxes);
    }
    Ok(())
}
