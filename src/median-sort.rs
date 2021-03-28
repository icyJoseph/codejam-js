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

fn list_to_string(ls: &Vec<usize>) -> String {
    ls.iter()
        .map(|x| (x + 1).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn query(x: usize, y: usize, z: usize) -> usize {
    println!("{} {} {}", x + 1, y + 1, z + 1);
    return parse_num::<usize>() - 1;
}

fn binary_search(body: &Vec<usize>, target: usize) -> usize {
    let lower = 0;
    let upper = body.len() / 2;

    if body.len() == 0 {
        return 1;
    }

    if body.len() == 1 {
        return 1;
    }

    if body.len() == 2 {
        let answer = query(body[lower], body[upper], target);

        if answer == target {
            return lower + 1;
        } else if answer == body[lower] {
            return lower;
        } else if answer == body[upper] {
            return upper + 1;
        }
    }

    let answer = query(body[lower], body[upper], target);

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

    let t = spec[0]; // total cases
    let n = spec[1]; // size

    for _ in 0..t {
        let mut head = 0;

        let mut body = vec![1];

        let mut next = 2;

        loop {
            let answer = query(head, body[0], next);

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

            if next == n {
                break;
            }
        }

        body.insert(0, head);

        println!("{}", list_to_string(&body));

        nxt();
    }

    Ok(())
}
