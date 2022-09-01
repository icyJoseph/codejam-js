use std::io;

/*
15012233444 -> chunk 3 4 4
150 1223 3444 ./ read

count 150 [[1,1], [5,1], [0,1]] [digit, contiguous count]
count 1223 [[1,1], [2,2], [3,1]]
count 3444 [[3,1], [4,3]]

read [[3,1], [4,3]] three triple four
*/

fn into_chunks(vec: &Vec<u32>, format: &Vec<usize>) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = vec![vec![]];

    let mut it = format.iter();
    let mut current_size = it.next();

    for &n in vec {
        let last_index = result.len() - 1;

        match current_size {
            Some(&size) => {
                if result[last_index].len() == size {
                    current_size = it.next();
                    result.push(vec![n]);
                } else {
                    result[last_index].push(n)
                }
            }
            _ => {}
        }
    }

    result
}

fn count(chunk: &Vec<u32>) -> Vec<(u32, u32)> {
    let mut result: Vec<(u32, u32)> = vec![];

    for &n in chunk {
        let len = result.len();
        if len == 0 {
            result.push((n, 1));
            continue;
        }

        let last_index = len - 1;

        let last = result[last_index];
        if last.0 == n {
            result[last_index].1 += 1;
        } else {
            result.push((n, 1));
        }
    }

    result
}

fn digit_word(digit: u32) -> String {
    let result = match digit {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        0 => "zero",
        _ => "",
    };

    String::from(result)
}

fn qualifier(count: u32) -> String {
    let result = match count {
        1 => "",
        2 => "double",
        3 => "triple",
        4 => "quadruple",
        5 => "quintuple",
        6 => "sextuple",
        7 => "septuple",
        8 => "octuple",
        9 => "nonuple",
        10 => "decuple",
        0 => panic!("count: {} is not supported", count),
        _ => "",
    };

    String::from(result)
}

fn read(pair: &(u32, u32)) -> String {
    let &(digit, count) = pair;

    if count == 0 {
        panic!("Zero count")
    }

    if count > 10 || count == 1 {
        let result = vec![digit_word(digit); count as usize];
        return string_vec(&result, " ");
    }

    return format!("{} {}", qualifier(count), digit_word(digit));
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let line = nxt();
        let spec = line.trim().split_ascii_whitespace().collect::<Vec<&str>>();

        let phone = spec[0]
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        let format = spec[1]
            .split("-")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let result = into_chunks(&phone, &format)
            .iter()
            .flat_map(|chunk| {
                count(chunk)
                    .iter()
                    .map(|pair| read(pair))
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<String>>();

        println!("Case #{}: {}", case, string_vec(&result, " "));
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
