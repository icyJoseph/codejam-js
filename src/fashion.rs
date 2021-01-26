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

    for r in 0..grid.len() {
        if r == entry.0 {
            for c in 0..grid[r].len() {
                if c != entry.1 { 
                    rc.push(grid[r][c]);
                 }
            }
        }

        rc.push(grid[r][entry.1]);
    }

    rc
}

fn dg(entry: (usize, usize), grid: &Vec<Vec<char>>, size: usize) -> Vec<(char, usize)> {
    let mut s_dg = vec![];
    let mut i = 0;

    loop {
        match entry.0.checked_sub(i) {
            Some(y) => match grid.get(y) {
                Some(row) => match row.get(entry.1 + i) {
                    Some(&c) => s_dg.push((c, fmod((y, entry.1 + i), size))),
                    None => break,
                },
                None => break,
            },
            None => break,
        }

        i += 1;
    }

    s_dg
}

fn fmod(entry: (usize, usize), size: usize) -> usize {
    let (row, col) = entry;
    row * size + col
}

fn rmod(m: usize, size: usize) -> (usize, usize) {
    (m / size, m % size)
}

fn remove_none_rows(grid: Vec<Vec<Option<(char, usize)>>>) -> Vec<Vec<Option<(char, usize)>>> {
    let mut result = vec![];
    for row in grid {
        if row.iter().all(|x| x.is_none()) {
            continue;
        }
        result.push(row);
    }

    result
}

fn shrink(grid: Vec<Vec<Option<(char, usize)>>>) -> Vec<Vec<Option<(char, usize)>>> {
    let mut shrunk: Vec<Vec<Option<(char, usize)>>> = vec![];

    let mut skip_r = vec![];
    let mut skip_c = vec![];

    for r in 0..grid.len() {
        match grid[r].iter().position(|x| match x {
            Some(('+', _)) => true,
            _ => false,
        }) {
            Some(c) => {
                skip_r.push(r);
                skip_c.push(c);
            }
            None => {}
        }
    }

    for r in 0..grid.len() {
        if skip_r.contains(&r) {
            continue;
        }

        let mut row = vec![];

        for c in 0..grid[r].len() {
            if skip_c.contains(&c) {
                continue;
            }
            let entry = grid[r][c];
            row.push(entry);
        }

        shrunk.push(row);
    }

    remove_none_rows(shrunk)
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

                            if p_grid[row][col] == '+' {
                                changes.insert(fmod(entry, size), 'o');
                            } else {
                                changes.insert(fmod(entry, size), 'x');
                            }
                        }
                    }
                    _ => panic!("Unknown model"),
                }
            }
        }

        let mut odd_grid: Vec<Vec<Option<(char, usize)>>> = vec![vec![None; size]; size];
        let mut even_grid: Vec<Vec<Option<(char, usize)>>> = vec![vec![None; size]; size];

        for row in 0..size {
            let s_dg = dg((row, 0), &p_grid, size);

            if row % 2 == 0 {
                let padding = (size - s_dg.len()) / 2;
                for index in padding..s_dg.len() + padding {
                    even_grid[row / 2][index] = Some(s_dg[index - padding]);
                }
            } else {
                let padding = (size - s_dg.len()) / 2;
                for index in padding..s_dg.len() + padding {
                    odd_grid[row / 2][index] = Some(s_dg[index - padding]);
                }
            }

            if row == size - 1 {
                for col in 1..size {
                    let s_dg = dg((row, col), &p_grid, size);

                    if (row + col) % 2 == 0 {
                        let padding = (size - s_dg.len()) / 2;
                        for index in padding..s_dg.len() + padding {
                            even_grid[(row + col) / 2][index] = Some(s_dg[index - padding]);
                        }
                    } else {
                        let padding = (size - s_dg.len()) / 2;
                        for index in padding..s_dg.len() + padding {
                            odd_grid[(row + col) / 2][index] = Some(s_dg[index - padding]);
                        }
                    }
                }
            }
        }

        while even_grid.iter().any(|row| {
            row.iter().any(|x| match x {
                Some(('+', _)) => true,
                _ => false,
            })
        }) {
            even_grid = shrink(even_grid);
        }

        even_grid = remove_none_rows(even_grid);

        loop {
            if even_grid.len() == 0 {
                break;
            }

            let next = {
                let mut ret = 0;

                let mut min = even_grid[0]
                    .iter()
                    .filter(|x| match x {
                        Some(_) => true,
                        _ => false,
                    })
                    .collect::<Vec<&Option<(char, usize)>>>()
                    .len();

                for r in 1..even_grid.len() {
                    let sat = even_grid[r]
                        .iter()
                        .filter(|x| match x {
                            Some(_) => true,
                            _ => false,
                        })
                        .collect::<Vec<&Option<(char, usize)>>>()
                        .len();

                    if sat < min {
                        min = sat;
                        ret = r;
                    }
                }
                ret
            };

            for col in 0..even_grid[next].len() {
                match even_grid[next][col] {
                    Some(('.', p)) => {
                        even_grid[next][col] = Some(('+', p));
                        points += 1;
                        let (r, c) = rmod(p, size);
                        match changes.get(&p) {
                            Some(&'x') => changes.insert(p, 'o'),
                            None if x_grid[r][c] == 'x' => changes.insert(p, 'o'),
                            None => changes.insert(p, '+'),
                            _ => panic!("Unknown symbol"),
                        };
                        break;
                    }
                    _ => continue,
                }
            }

            if !even_grid.iter().any(|row| {
                row.iter().any(|x| match x {
                    Some(('+', _)) => true,
                    _ => false,
                })
            }) {
                break;
            }

            even_grid = shrink(even_grid);
        }

        while odd_grid.iter().any(|row| {
            row.iter().any(|x| match x {
                Some(('+', _)) => true,
                _ => false,
            })
        }) {
            odd_grid = shrink(odd_grid);
        }

        odd_grid = remove_none_rows(odd_grid);

        loop {
            if odd_grid.len() == 0 {
                break;
            }

            let next = {
                let mut ret = 0;

                let mut min = odd_grid[0]
                    .iter()
                    .filter(|x| match x {
                        Some(_) => true,
                        _ => false,
                    })
                    .collect::<Vec<&Option<(char, usize)>>>()
                    .len();

                for r in 1..odd_grid.len() {
                    let sat = odd_grid[r]
                        .iter()
                        .filter(|x| match x {
                            Some(_) => true,
                            _ => false,
                        })
                        .collect::<Vec<&Option<(char, usize)>>>()
                        .len();

                    if sat < min {
                        min = sat;
                        ret = r;
                    }
                }
                ret
            };

            for col in 0..odd_grid[next].len() {
                match odd_grid[next][col] {
                    Some(('.', p)) => {
                        odd_grid[next][col] = Some(('+', p));
                        points += 1;
                        let (r, c) = rmod(p, size);
                        match changes.get(&p) {
                            Some(&'x') => changes.insert(p, 'o'),
                            None if x_grid[r][c] == 'x' => changes.insert(p, 'o'),
                            None => changes.insert(p, '+'),
                            _ => panic!("Unknown symbol"),
                        };
                        break;
                    }
                    _ => continue,
                }
            }

            if !odd_grid.iter().any(|row| {
                row.iter().any(|x| match x {
                    Some(('+', _)) => true,
                    _ => false,
                })
            }) {
                break;
            }

            odd_grid = shrink(odd_grid);
        }

        println!("Case #{}: {} {}", case, points, changes.len());

        for (key, val) in changes {
            let (r, c) = rmod(key, size);
            println!("{} {} {}", val, r + 1, c + 1);
        }
    }
    Ok(())
}
