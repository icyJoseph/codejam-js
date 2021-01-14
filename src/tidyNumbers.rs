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
        let mut num: Vec<u32> = nxt()
            .trim()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .rev()
            .collect();

        let l = num.len();

        for i in 0..l - 1 {
            let next = num[i + 1];
            if next > num[i] {
                // untidy
                num[i] = 9;
                num[i + 1] = if next == 0 { 9 } else { next - 1 };

                for j in (0..i).rev() {
                    if num[j] < num[i] {
                        num[j] = 9
                    }
                }
            }
        }

        let tidy: u64 = num
            .into_iter()
            .rev()
            .map(|x| x as u64)
            .fold(0, |prev, curr| curr + prev * 10);

        println!("Case #{}: {:?}", case, tidy);
    }
    
    Ok(())
}
