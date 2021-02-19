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
        let spec = parse_vec::<u32>();

        let total = spec[0] as usize;
        let max = spec[1];

        let mut amounts = parse_vec::<u32>();
        let mut people = (0..total).collect::<Vec<usize>>();
        let mut exit = vec![];

        loop {
            if people.len() == 0 {
                break;
            }
            let next = people.remove(0);
            let next_amount = amounts[next];

            if next_amount > max {
                people.push(next);
                amounts[next] = next_amount - max;
            } else {
                exit.push(next);
            }
        }

        let answer = format!(
            "{}",
            exit.iter()
                .map(|x| (x + 1).to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );

        println!("Case #{}: {}", case, answer);
    }
    Ok(())
}
