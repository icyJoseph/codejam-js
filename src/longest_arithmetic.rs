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
        let total = parse_num::<usize>();

        let numbers = parse_vec::<i64>();

        let mut diff = numbers[1] - numbers[0];

        let mut current = 2;
        let mut longest = 0;

        for pos in 1..total - 1 {
            let curr = numbers[pos];
            let next = numbers[pos + 1];

            if next - curr == diff {
                current += 1;
            } else {
                diff = next - curr;
                if current > longest {
                    longest = current;
                }
                current = 2;
            }
        }

        if current > longest {
            longest = current;
        }

        println!("Case #{}: {}", case, longest);
    }
    Ok(())
}
