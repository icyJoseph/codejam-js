use std::io;

/**
 N blocks

ABCDEF.....

from L to R, inclusive
is it possible to form a palindrome?
using all blacks in the L, R range?

aba
abba

a

if L+1 == R, yes

other cases,

ab
abc
abcd

to be palindromes require

ab -> a = b
abc -> a = c
abcd -> a = d and b = c

 */

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let spec = parse_vec::<usize>();
        let blocks_raw = nxt();
        let blocks = blocks_raw
            .trim()
            .chars()
            .map(|c| c as u8)
            .collect::<Vec<_>>();

        use std::collections::HashMap;

        let mut frequencies: Vec<Vec<usize>> = vec![];

        let mut prev: Vec<usize> = vec![];

        let mut lookup: HashMap<u8, usize> = HashMap::new();

        // 'A'..='Z' is not supported in CodeJam's Rust version
        for (index, value) in (b'A'..=b'Z').enumerate() {
            prev.push(0);
            lookup.insert(value, index);
        }

        frequencies.push(prev);

        for value in blocks.iter() {
            let mut current = frequencies[frequencies.len() - 1].to_vec();

            match lookup.get(value) {
                Some(&index) => {
                    current[index] = current[index] + 1;
                }
                _ => {}
            }

            frequencies.push(current);
        }

        let mut total = 0;

        for _ in 0..spec[1] {
            let range = parse_vec::<usize>();

            if range[0] == range[1] {
                total = total + 1;
                continue;
            }

            let l = range[0] - 1;
            let r = range[1];

            let mut odd_presence = 0;

            // 'A'..='Z' is not supported in CodeJam's Rust version
            for value in b'A'..=b'Z' {
                if let Some(&index) = lookup.get(&value) {
                    let right = frequencies[r][index];
                    let left = frequencies[l][index];

                    if (right - left) % 2 != 0 {
                        odd_presence += 1;
                    }
                }
            }

            if odd_presence <= 1 {
                total = total + 1;
            }
        }

        println!("Case #{}: {}", case, total);
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
