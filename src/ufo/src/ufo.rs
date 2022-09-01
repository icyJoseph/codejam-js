// https://codingcompetitions.withgoogle.com/codejam/round/00000000000000cb/00000000000079cc#problem
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

fn pretty_print(v: Vec<f64>) -> String {
    let mut answer = String::new();
    for entry in v.iter() {
        answer = format!("{} {}", answer, entry);
    }
    answer.trim().to_string()
}

fn mat_mult(m: Vec<Vec<f64>>, v: Vec<f64>) -> Vec<f64> {
    let mut ret = vec![];

    for row in m.iter() {
        let mut val = 0.0;
        for (k, cell) in row.iter().enumerate() {
            val = cell * v[k] + val;
        }
        ret.push(val);
    }

    ret
}

fn rot(v: Vec<f64>, angle: f64) -> Vec<f64> {
    // 45 degrees rotation around the Y axis
    // this way the projection is still area 1
    let k = (2.0_f64).sqrt() / 2.0;
    let pitch = vec![vec![k, 0.0, k], vec![0.0, 1.0, 0.0], vec![-k, 0.0, k]];

    // compensation yaw rotation to achieve area (X axis)
    let sin = angle.sin();
    let cos = angle.cos();

    let yaw = vec![
        vec![cos, -sin, 0.0],
        vec![sin, cos, 0.0],
        vec![0.0, 0.0, 1.0],
    ];

    mat_mult(yaw, mat_mult(pitch, v))
}

fn main() -> Res<()> {
    let n = ptc::<i32>();
    for case in 1..=n {
        let s = ptc::<f64>();
        let d = (3.0_f64).sqrt();

        let start = (1.0 / d).asin();
        let angle = (s / d).asin() - start;

        println!("Case #{}:", case);
        println!("{}", pretty_print(rot(vec![0.5, 0.0, 0.0], angle)));
        println!("{}", pretty_print(rot(vec![0.0, 0.5, 0.0], angle)));
        println!("{}", pretty_print(rot(vec![0.0, 0.0, 0.5], angle)));
    }
    Ok(())
}
