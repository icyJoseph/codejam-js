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
        nxt();
        let next: Vec<u64> = nxt()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let mut odd = vec![];
        let mut even = vec![];

        for (index, entry) in next.into_iter().enumerate() {
            if index % 2 == 0 {
                even.push(entry);
            } else {
                odd.push(entry)
            };
        }

        odd.sort();
        even.sort();

        let mut answer = format!("Case #{}: OK", case);

        for (index, entry) in even.iter().enumerate() {
            match odd.get(index) {
                Some(val) if entry > val => {
                    answer = format!("Case #{}: {}", case, 2 * index);
                    break;
                }
                Some(val) if entry <= val => match even.get(index + 1) {
                    Some(third) if third < val => {
                        answer = format!("Case #{}: {}", case, 2 * index + 1);
                        break;
                    }
                    _ => continue,
                },
                _ => continue,
            };
        }
        println!("{}", answer);
    }

    Ok(())
}
