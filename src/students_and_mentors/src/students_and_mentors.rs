use std::io;

// check if the middle element
// Rm <= 2 * Rs
// and that is not the same
fn search(student: (usize, &u64), list: &[(usize, &u64)]) -> Option<usize> {
    let max_mentor = 2 * student.1;
    let position = student.0;

    if list.len() < 100 {
        return match list
            .iter()
            .rposition(|(index, &rating)| max_mentor >= rating && *index != position)
        {
            Some(index) => Some(list[index].0),
            None => None,
        };
    }

    let pivot = list.len() / 2;

    let candidate = list[pivot];

    if *candidate.1 > max_mentor {
        // candidate is too large, search on the lower half
        return search(student, &list[..pivot + 1]);
    }

    // search on the upper half
    return search(student, &list[pivot..]);
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let _total = parse_num::<u64>();
        let students = parse_vec::<u64>();

        let mut list: Vec<(usize, &u64)> = students.iter().enumerate().collect::<_>();

        list.sort_by(|a, b| a.1.cmp(&b.1));

        let answer = students
            .iter()
            .enumerate()
            .map(|(index, rating)| search((index, rating), &list))
            .map(|result| match result {
                Some(n) => {
                    format!("{}", students[n])
                }
                None => "-1".to_string(),
            })
            .collect::<Vec<_>>();

        println!("Case #{}: {}", case, string_vec(&answer, " "));
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
