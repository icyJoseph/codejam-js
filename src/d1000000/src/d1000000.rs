use std::io;

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let total_dice = parse_num::<isize>();
        let mut dice = parse_vec::<isize>();

        if total_dice == 1 {
            println!("Case #{}: {}", case, 1);
        } else {
            dice.sort();

            let hist = dice
                .iter()
                .enumerate()
                .map(|(idx, value)| value - (idx as isize + 1))
                .collect::<Vec<isize>>();

            let lowest_hist = hist.iter().min().unwrap();

            if *lowest_hist > 0 {
                println!("Case #{}: {}", case, total_dice);
            } else {
                println!("Case #{}: {}", case, total_dice + lowest_hist);
            }
        }
    }

    Ok(())
}

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

#[allow(dead_code)]
fn parse_vec<T: std::str::FromStr>() -> Vec<T> {
    nxt()
        .split_whitespace()
        .map(|x| match x.parse::<T>() {
            Ok(n) => n,
            _ => panic!("Could not parse vector"),
        })
        .collect()
}

#[allow(dead_code)]
fn string_vec<T: std::string::ToString>(vec: &Vec<T>, separator: &str) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(separator)
}
