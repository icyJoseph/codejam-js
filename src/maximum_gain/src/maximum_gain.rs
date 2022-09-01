use std::collections::HashMap;
use std::io;

fn walk_list(list: &Vec<usize>, limit: usize, predicate: &mut dyn FnMut(usize, usize)) {
    let total = list.iter().sum::<usize>();

    for start in 0..=list.len() {
        let mut end_acc = 0;

        for end in start..=list.len() {
            let task = start + (list.len() - end);

            if task <= limit {
                let value = total - end_acc;

                predicate(task, value);
            }

            if end == list.len() {
                continue;
            }

            end_acc += list[end];
        }
    }
}

fn compile_map(list: &Vec<usize>, limit: usize) -> HashMap<usize, usize> {
    let mut map: HashMap<usize, usize> = HashMap::new();

    let mut update_map = |task, value| {
        if task <= limit {
            match map.get(&task) {
                None => {
                    map.insert(task, value);
                }
                Some(&prev) => {
                    if value > prev {
                        map.insert(task, value);
                    }
                }
            }
        }
    };

    walk_list(list, limit, &mut update_map);

    map
}

fn compile_complement(
    list: &Vec<usize>,
    limit: usize,
    source_map: HashMap<usize, usize>,
) -> Option<usize> {
    let mut max = None;

    let mut update_max = |task, value| {
        let c_k = limit - task;

        match source_map.get(&c_k) {
            None => {
                // do nothing
            }
            Some(&prev) => match max {
                Some(m) if m < prev + value => max = Some(prev + value),
                None => max = Some(prev + value),
                _ => {}
            },
        }
    };

    walk_list(list, limit, &mut update_max);

    max
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let _total = parse_num::<usize>();
        let left_tasks = parse_vec::<usize>();

        let _total = parse_num::<usize>();
        let right_tasks = parse_vec::<usize>();

        let max_tasks = parse_num::<usize>();

        let source_map = compile_map(&left_tasks, max_tasks);

        match compile_complement(&right_tasks, max_tasks, source_map) {
            Some(value) => {
                println!("Case #{}: {}", case, value);
            }
            _ => {}
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
