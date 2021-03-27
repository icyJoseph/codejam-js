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

fn print_ls(ls: &Vec<usize>) -> String {
    ls.iter()
        .map(|x| (x + 1).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn print_triplet(triplet: &(usize, usize, usize)) -> String {
    format!("{} {} {}", triplet.0 + 1, triplet.1 + 1, triplet.2 + 1)
}

fn binary_search(body: &Vec<usize>, target: usize) -> usize {
    let mut lower = 0;
    let mut upper = body.len() / 2;

    if body.len() == 0 {
        return 1;
    }

    if body.len() == 1 {
        return 1;
    }

    if body.len() == 2 {
        println!("{}", print_triplet(&(body[lower], body[upper], target)));

        let answer = parse_num::<usize>() - 1;

        if answer == target {
            return lower + 1;
        } else if answer == body[lower] {
            return lower;
        } else if answer == body[upper] {
            return upper + 1;
        }
    }

    println!("{}", print_triplet(&(body[lower], body[upper], target)));

    let answer = parse_num::<usize>() - 1;

    if answer == target {
        // target is between lower and upper
        let index = binary_search(&body[lower..upper].to_vec(), target);
        return lower + index;
    } else if answer == body[lower] {
        return 0;
    } else if answer == body[upper] {
        // c must be between upper and the end of body
        let index = binary_search(&body[upper..].to_vec(), target);
        return upper + index;
    } else {
        panic!("Unexpected result");
    }
}

fn main() -> Res<()> {
    let spec = parse_vec::<usize>();

    let T = spec[0]; // total cases
    let N = spec[1]; // size
    let Q = spec[2]; // queries

    for case in 0..T {
        let mut head = 0;

        let mut body = vec![1];

        let mut next = 2;

        loop {
            println!("{}", print_triplet(&(head, body[0], next)));

            let answer = parse_num::<usize>() - 1;

            if answer == body[0] {
                let index = binary_search(&body, next);
                body.insert(index, next);
            } else if answer == head {
                body.insert(0, head);
                head = next;
            } else {
                // next is median
                body.insert(0, next);
            }

            next += 1;

            if next == N {
                break;
            }
        }

        body.insert(0, head);

        println!("{}", print_ls(&body));

        nxt();
    }

    Ok(())
}
