use std::char::from_digit;
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

fn to_digits(n: u128) -> Vec<char> {
    n.to_string().chars().collect()
}

fn left_gt_right(left: &Vec<char>, right: &Vec<char>) -> bool {
    if right.len() > left.len() {
        return false;
    } else if left.len() > right.len() {
        return true;
    }

    for i in 0..left.len() {
        if right[i] == left[i] {
            continue;
        } else if right[i] > left[i] {
            return false;
        } else {
            return true;
        }
    }

    return false;
}

fn add_one(vec: &Vec<char>) -> Vec<char> {
    let mut copy = vec.to_vec();

    let mut carry = 1;

    for i in (0..copy.len()).rev() {
        let num = copy[i];

        if num == '9' && carry == 1 {
            copy[i] = '0';
            carry = 1;
        } else {
            copy[i] = from_digit(num.to_digit(10).unwrap() + carry, 10).unwrap();
            carry = 0;
        }
    }

    copy
}

fn overtake(curr: &Vec<char>, prev: &Vec<char>) -> Vec<char> {
    let mut curr_digits = curr.to_vec();

    let diff = prev.len() - curr_digits.len();

    if diff == 0 {
        curr_digits.push('0');
        return curr_digits;
    }

    // push diff 0's -> if larger then prev, return
    for _ in 0..diff {
        curr_digits.push('0');
    }

    if left_gt_right(&curr_digits, prev) {
        return curr_digits;
    }

    let mut nines = curr.to_vec();

    // push diff 9's ->
    for _ in 0..diff {
        nines.push('9');
    }

    if left_gt_right(&nines, prev) {
        return add_one(&prev);
    } else {
        let mut next = curr.to_vec();
        for _ in 0..diff + 1 {
            next.push('0');
        }
        return next;
    }
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let total = parse_num::<usize>();

        let num_list = parse_vec::<u128>();

        let mut list = num_list
            .iter()
            .map(|&x| to_digits(x))
            .collect::<Vec<Vec<char>>>();

        let mut changes = 0;

        for index in 1..total {
            let prev = list[index - 1].to_vec();
            let curr = list[index].to_vec();

            if left_gt_right(&curr, &prev) {
                continue;
            }

            let next = overtake(&curr, &prev);

            changes += next.len() - curr.len();

            list[index] = next;
        }

        println!("Case #{}: {}", case, changes);
    }
    Ok(())
}
