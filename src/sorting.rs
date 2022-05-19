use std::io;

/*
5 2 4 3 1

Alex 5 3 1 -> (lr inc) -> 1 3 5

Bob 2 4 -> (lr dec) -> 4 2

[A, B, B, A, A]
[1, 4, 2, 3, 5]

*/

enum Position {
    Alex,
    Bob,
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let _total = parse_num::<usize>();

        let books = parse_vec::<i32>();

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut alex = BinaryHeap::new();
        let mut bob = BinaryHeap::new();

        let mut shelf = vec![];

        for book in books {
            if book % 2 == 0 {
                shelf.push(Position::Bob);
                bob.push(book);
            } else {
                shelf.push(Position::Alex);
                alex.push(Reverse(book));
            }
        }

        let mut result = vec![];

        for position in shelf {
            match position {
                Position::Alex => match alex.pop() {
                    Some(Reverse(n)) => result.push(n),
                    _ => {}
                },
                Position::Bob => match bob.pop() {
                    Some(n) => result.push(n),
                    _ => {}
                },
            }
        }

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
