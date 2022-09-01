// https://codingcompetitions.withgoogle.com/codejam/round/000000000019fd27/000000000020993c
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

fn main() -> Res<()> {
    let n = ptc::<i32>();

    for case in 1..=n {
        let rows = ptc::<usize>();

        let mut trace = 0;

        let mut grid: Vec<Vec<i32>> = vec![vec![0; rows]; rows];

        use std::collections::HashSet;

        let mut rep_rows: HashSet<usize> = HashSet::new();
        let mut rep_cols: HashSet<usize> = HashSet::new();

        for row in 0..rows {
            for (i, cell) in nxt().split_whitespace().enumerate() {
                let val = cell.parse::<i32>()?;
                if grid[row].contains(&val) {
                    rep_rows.insert(row);
                };

                let col = grid.iter().map(|r| r[i]).collect::<Vec<i32>>();

                if col.contains(&val) {
                    rep_cols.insert(i);
                };

                grid[row][i] = val;

                if row == i {
                    trace = trace + val
                };
            }
        }

        println!(
            "Case #{}: {} {} {}",
            case,
            trace,
            rep_rows.len(),
            rep_cols.len()
        );
    }

    Ok(())
}
