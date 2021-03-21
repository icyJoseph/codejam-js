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
        let spec = parse_vec::<usize>();

        let r = spec[0];
        let c = spec[1];

        let mut grid = vec![];

        for _ in 0..r {
            let row = parse_vec::<u32>();
            grid.push(row);
        }

        let mut blocks_x: Vec<Vec<(usize, usize)>> = vec![];

        for i in 0..r {
            blocks_x.push(vec![]);
            for j in 0..c {
                let cell = grid[i][j];

                let last = blocks_x.pop();

                match last {
                    Some(mut v) if cell == 1 => {
                        v.push((i, j));
                        blocks_x.push(v);
                    }
                    Some(v) => {
                        blocks_x.push(v);
                        if j < c {
                            blocks_x.push(vec![]);
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut blocks_y: Vec<Vec<(usize, usize)>> = vec![];

        for j in 0..c {
            blocks_y.push(vec![]);
            for i in 0..r {
                let cell = grid[i][j];

                let last = blocks_y.pop();

                match last {
                    Some(mut v) if cell == 1 => {
                        v.push((i, j));
                        blocks_y.push(v);
                    }
                    Some(v) => {
                        blocks_y.push(v);
                        if j < c {
                            blocks_y.push(vec![]);
                        }
                    }
                    _ => {}
                }
            }
        }

        let valid_blocks_x: Vec<&Vec<(usize, usize)>> =
            blocks_x.iter().filter(|x| x.len() > 1).collect();

        let valid_blocks_y: Vec<&Vec<(usize, usize)>> =
            blocks_y.iter().filter(|x| x.len() > 1).collect();

        //Each of the segments must be a "good" segment.
        //The two segments must be perpendicular to each other.
        //The segments must share one cell that is an endpoint of both segments.
        //Segments must have length at least 2.
        //The length of the longer segment is twice the length of the shorter segment.
        //

        let mut inter = vec![];

        for block in valid_blocks_x.iter() {
            for (i, point) in block.iter().enumerate() {
                // is it an endpoint of a block in valid_blocks_y?
                let up = block.len() - i + 1;
                let down = i + 1;

                for candidate in valid_blocks_y.iter() {
                    let l = candidate.len();
                    let left = candidate[0];
                    let right = candidate[l - 1];

                    if left.0 == point.0 && left.1 == point.1 {
                        let xy = (point.0, point.1, up, down, l);
                        inter.push(xy);
                    } else if right.0 == point.0 && right.1 == point.1 {
                        let xy = (point.0, point.1, up, down, l);
                        inter.push(xy);
                    } else if i == 0 || i == block.len() - 1 {
                        if block.len() >= 2 {
                            for y_block in candidate.iter() {
                                if y_block.0 == point.0 && y_block.1 == point.1 {
                                    let xy = (point.0, point.1, up, down, block.len());
                                    inter.push(xy);
                                }
                            }
                        }
                    }
                }
            }
        }

        let mut count = 0;
        for xy in inter.iter() {
            let l = xy.4;
            let up = xy.2;
            let down = xy.3;

            for s in 1..l {
                let doubled = s * 2;
                let halved = s / 2;

                if doubled <= up {
                    count += 1;
                } else if doubled <= down {
                    count += 1;
                } else if halved > 1 && halved <= up {
                    count += 1;
                } else if halved > 1 && halved <= down {
                    count += 1;
                }
            }
        }

        println!("Case #{}: {}", case, count);
    }
    Ok(())
}
