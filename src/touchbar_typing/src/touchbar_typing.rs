use std::cmp::{max, min};
use std::collections::HashMap;
use std::io;

fn calc_distance(a: usize, b: usize) -> usize {
    max(a, b) - min(a, b)
}

fn find_closest(target: usize, list: &[usize]) -> (Option<usize>, Option<usize>) {
    if list.len() == 1 {
        if target < list[0] {
            return (None, Some(list[0]));
        }

        return (Some(list[0]), None);
    }
    if list.len() == 2 {
        if target < list[1] && target > list[0] {
            return (Some(list[0]), Some(list[1]));
        }
        if target <= list[0] {
            return (None, Some(list[0]));
        }
        if target >= list[1] {
            return (Some(list[1]), None);
        }
    }

    let half = list.len() / 2;
    let pivot = list[half];

    if target == pivot {
        return (Some(pivot), None);
    }
    if target > pivot {
        return find_closest(target, &list[half..]);
    }

    return find_closest(target, &list[..=half]);
}

// fn type_word(
//     position: usize,
//     current_key: usize,
//     rest: &[usize],
//     path_finder: &HashMap<String, (Option<usize>, Option<usize>)>,
// ) -> usize {
//     if rest.len() == 0 {
//         return 0;
//     }

//     let key = format!("{}.{}.{}", current_key, position, rest[0]);

//     let (left_opt, right_opt) = path_finder.get(&key).unwrap();

//     match (left_opt, right_opt) {
//         (&Some(left), &Some(right)) => {
//             let left_branch =
//                 calc_distance(left, position) + type_word(left, rest[0], &rest[1..], path_finder);

//             let right_branch =
//                 calc_distance(right, position) + type_word(right, rest[0], &rest[1..], path_finder);

//             min(left_branch, right_branch)
//         }
//         (None, &Some(right)) => {
//             let result =
//                 calc_distance(right, position) + type_word(right, rest[0], &rest[1..], path_finder);

//             result
//         }
//         (&Some(left), None) => {
//             let result =
//                 calc_distance(left, position) + type_word(left, rest[0], &rest[1..], path_finder);

//             result
//         }
//         _ => {
//             panic!("next not found")
//         }
//     }
// }

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let _word_len = parse_num::<usize>();

        let word = parse_vec::<usize>()
            .iter()
            .map(|&x| x - 1)
            .collect::<Vec<_>>();

        let _keyboard_len = parse_num::<usize>();

        let keyboard = parse_vec::<usize>()
            .iter()
            .map(|&x| x - 1)
            .collect::<Vec<_>>();

        // table maps key to indexes where it is found in keyboard
        let mut table: HashMap<usize, Vec<usize>> = HashMap::new();

        for (index, &value) in keyboard.iter().enumerate() {
            let current = table.entry(value).or_insert(vec![]);
            current.push(index);
        }

        // for a `current_key` at a `position in keyboard` with destination `next_key`, what are
        // the best options
        // let mut path_finder: HashMap<String, (Option<usize>, Option<usize>)> = HashMap::new();

        // for (pos, current_key) in word[..word.len() - 1].iter().enumerate() {
        //     let next_key = word[pos + 1];

        //     // best options from current key and position to next key
        //     match table.get(current_key) {
        //         Some(choices) => {
        //             for &position in choices {
        //                 match table.get(&next_key) {
        //                     Some(destinations) => {
        //                         let paths = find_closest(position, &destinations);

        //                         let key = format!("{}.{}.{}", current_key, position, next_key);

        //                         path_finder.insert(key, paths);
        //                     }
        //                     _ => {}
        //                 }
        //             }
        //         }

        //         None => {}
        //     }
        // }

        let mut state: Vec<Vec<usize>> = vec![vec![0; keyboard.len()]; word.len()];

        for (index, &letter) in word.iter().enumerate() {
            if let Some(choices) = table.get(&letter) {
                if index == 0 {
                    continue;
                }
                // all positions of letter
                let prev_letter = word[index - 1];

                for &choice in choices.iter() {
                    if let Some(prev_positions) = table.get(&prev_letter) {
                        let prev_optimal = find_closest(choice, &prev_positions);
                        let mut acc = vec![];

                        match prev_optimal {
                            (Some(left), Some(right)) => {
                                acc.push(calc_distance(choice, left) + state[index - 1][left]);

                                acc.push(calc_distance(choice, right) + state[index - 1][right]);
                            }

                            (Some(left), None) => {
                                acc.push(calc_distance(choice, left) + state[index - 1][left]);
                            }

                            (None, Some(right)) => {
                                acc.push(calc_distance(choice, right) + state[index - 1][right]);
                            }
                            _ => {}
                        }

                        // for &prev_position in prev_positions.iter() {
                        //     acc.push(
                        //         calc_distance(choice, prev_position)
                        //             + state[index - 1][prev_position],
                        //     )
                        // }
                        state[index][choice] = *acc.iter().min().unwrap();
                    }
                }
            }
        }

        if let Some(end_positions) = table.get(&word[word.len() - 1]) {
            let mut acc = vec![];

            for &position in end_positions {
                acc.push(state[word.len() - 1][position])
            }

            match acc.iter().min() {
                Some(value) => {
                    println!("Case #{}: {}", case, value);
                }
                _ => {}
            }
        }

        // match table.get(&word[0]) {
        //     Some(options) => {
        //         for &choice in options {
        //             println!("{}", choice);
        //             let result = type_word(choice, word[0], &word[1..], &path_finder);

        //             match shortest {
        //                 None => shortest = Some(result),
        //                 Some(prev) if result < prev => {
        //                     shortest = Some(result);
        //                 }
        //                 _ => {}
        //             }
        //         }
        //     }
        //     _ => {}
        // }
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
