// https://codingcompetitions.withgoogle.com/codejam/round/00000000002017f7/00000000002017f8#problem
use std::io;

type Res<T> = Result<T, Box<dyn std::error::Error>>;

fn nxt() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        _ => panic!("Error reading line"),
    }
}

fn ptc<T: std::str::FromStr>() -> T {
    match nxt().trim().parse::<T>() {
        Ok(n) => n,
        _ => panic!("Error parsing"),
    }
}

fn get_rc(entry: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<char> {
    let mut rc: Vec<char> = vec![];

    let mut i = 1;

    loop {
        let mut none = 0;

        // up
        match entry.0.checked_sub(i) {
            Some(y) => match grid.get(y) {
                Some(row) => match row.get(entry.1) {
                    Some(&c) => rc.push(c),
                    None => none += 1,
                },
                None => none += 1,
            },
            None => none += 1,
        }

        // down
        match grid.get(entry.0 + i) {
            Some(row) => match row.get(entry.1) {
                Some(&c) => rc.push(c),
                None => none += 1,
            },
            None => none += 1,
        }

        // left
        match grid.get(entry.0) {
            Some(row) => match row.get(entry.1 + i) {
                Some(&c) => rc.push(c),
                None => none += 1,
            },
            None => none += 1,
        }

        // right
        match grid.get(entry.0) {
            Some(row) => match entry.1.checked_sub(i) {
                Some(x) => match row.get(x) {
                    Some(&c) => rc.push(c),
                    None => none += 1,
                },
                None => none += 1,
            },
            None => none += 1,
        }

        if none == 4 {
            break;
        }

        i += 1;
    }

    rc
}

fn get_dg(entry: (usize, usize), grid: &Vec<Vec<char>>) -> Vec<char> {
    let mut dg: Vec<char> = vec![];

    let mut i = 1;

    loop {
        let mut none = 0;

        // down right
        match grid.get(entry.0 + i) {
            Some(row) => match row.get(entry.1 + i) {
                Some(&c) => dg.push(c),
                None => none += 1,
            },
            None => none += 1,
        }

        //  up right
        match entry.0.checked_sub(i) {
            Some(y) => match grid.get(y) {
                Some(row) => match row.get(entry.1 + i) {
                    Some(&c) => dg.push(c),
                    None => none += 1,
                },
                None => none += 1,
            },
            None => none += 1,
        }

        // down left
        match grid.get(entry.0 + i) {
            Some(row) => match entry.1.checked_sub(i) {
                Some(x) => match row.get(x) {
                    Some(&c) => dg.push(c),
                    None => none += 1,
                },
                None => none += 1,
            },
            None => none += 1,
        }

        // up left
        match entry.0.checked_sub(i) {
            Some(y) => match grid.get(y) {
                Some(row) => match entry.1.checked_sub(i) {
                    Some(x) => match row.get(x) {
                        Some(&c) => dg.push(c),
                        None => none += 1,
                    },
                    None => none += 1,
                },
                None => none += 1,
            },
            None => none += 1,
        }

        if none == 4 {
            break;
        }

        i += 1;
    }

    dg
}

fn fmod(entry: (usize, usize), size: usize) -> usize {
    let (row, col) = entry;
    row * size + col
}

fn rmod(m: usize, size: usize) -> (usize, usize) {
    (m / size, m % size)
}

fn main() -> Res<()> {
    let n = ptc::<i32>();

    for case in 1..=n {
        let spec: Vec<usize> = nxt()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let size = spec[0];
        let models = spec[1];

        let mut points = 0;
        let mut p_grid: Vec<Vec<char>> = vec![vec!['.'; size]; size];
        let mut x_grid: Vec<Vec<char>> = vec![vec!['.'; size]; size];

        for _ in 0..models {
            let next = nxt();
            let mut model_spec = next.trim().split_whitespace();

            let value = model_spec.next().unwrap().chars().next().unwrap();
            let r = model_spec.next().unwrap().parse::<usize>().unwrap() - 1;
            let c = model_spec.next().unwrap().parse::<usize>().unwrap() - 1;

            match value {
                '+' => {
                    p_grid[r][c] = value;
                    points += 1;
                }
                'x' => {
                    x_grid[r][c] = value;
                    points += 1;
                }
                'o' => {
                    p_grid[r][c] = '+';
                    x_grid[r][c] = 'x';
                    points += 2;
                }
                _ => panic!("Unknown model"),
            }
        }

        use std::collections::HashMap;

        let mut changes: HashMap<usize, char> = HashMap::new();

        for row in 0..size {
            for col in 0..size {
                let entry = (row, col);

                match x_grid[row][col] {
                    'x' => {}
                    '.' => {
                        let rc = get_rc(entry, &x_grid);
                        if !rc.contains(&'x') {
                            x_grid[row][col] = 'x';
                            points += 1;

                            changes.insert(fmod(entry, size), 'x');
                        }
                    }
                    _ => panic!("Unknown model"),
                }
            }
        }

        for row in 0..size {
            for col in 0..size {
                let entry = (row, col);

                if row == 0 || row == size - 1 {
                    // solve for p_grid
                    match p_grid[row][col] {
                        '+' => {}
                        '.' => {
                            let dg = get_dg(entry, &p_grid);

                            if !dg.contains(&'+') {
                                p_grid[row][col] = '+';
                                points += 1;

                                let key = fmod(entry, size);

                                match changes.get(&key) {
                                    Some(_) => changes.insert(key, 'o'),
                                    None => changes.insert(key, '+'),
                                };
                            }
                        }
                        _ => panic!("Unknown model"),
                    }
                }
            }
        }

        println!("Case #{}: {} {}", case, points, changes.len());

        for (key, val) in changes {
            let (r, c) = rmod(key, size);
            println!("{} {} {}", val, r + 1, c + 1);
        }
    }
    Ok(())
}
