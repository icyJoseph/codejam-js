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

fn flip(c: char) -> char {
    match c {
        '-' => '+',
        '+' => '-',
        _ => panic!("Error"),
    }
}

fn main() -> Res<()> {
    let n = ptc::<i32>();
    for case in 1..=n {
        let c: Vec<String> = nxt().split_whitespace().map(|x| x.to_string()).collect();

        let mut it: Vec<char> = c[0].chars().collect();
        let k = c[1].parse::<usize>().unwrap();

        let l = it.len();

        let mut acc = 0;

        for i in 0..l {
            match it[i] {
                '-' => {
                    if i > l - k {
                        break;
                    }
                    for j in i..(i + k) {
                        it[j] = flip(it[j]);
                    }
                    acc += 1;
                }
                _ => continue,
            }
        }

        let mut ans = format!("Case #{}: {}", case, acc);

        if it.contains(&'-') {
            ans = format!("Case #{}: {}", case, "IMPOSSIBLE");
        }
        println!("{}", ans);
    }
    Ok(())
}
