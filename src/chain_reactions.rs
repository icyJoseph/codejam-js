use std::cell::Cell;
use std::cmp::max;
use std::io;

#[derive(Debug)]
struct Node {
    id: usize,
    value: usize,
    output: usize,
    is_trigger: bool,
    is_preview: Cell<bool>,
    is_sealed: Cell<bool>,
}

impl Node {
    fn new(id: usize, value: usize, output: usize, is_trigger: bool) -> Self {
        Node {
            id,
            value,
            output,
            is_trigger,
            is_preview: Cell::new(false),
            is_sealed: Cell::new(false),
        }
    }

    fn calc(&self, rest: &Vec<Node>) -> usize {
        if self.output == 0 {
            return self.value;
        }

        let next = rest.iter().find(|n| n.id == self.output).unwrap();

        if next.is_sealed.get() {
            return self.value;
        }

        max(self.value, next.calc(rest))
    }

    fn preview_calc(&self, rest: &Vec<Node>) -> usize {
        if self.output == 0 {
            return self.value;
        }

        let next = rest.iter().find(|n| n.id == self.output).unwrap();

        if next.is_preview.get() || next.is_sealed.get() {
            return self.value;
        }

        max(self.value, next.preview_calc(rest))
    }

    fn set_preview(&self, rest: &Vec<Node>) {
        self.is_preview.set(true);

        if self.output == 0 {
            return ();
        }

        let next = rest.iter().find(|n| n.id == self.output).unwrap();

        next.set_preview(rest)
    }

    fn reset_preview(&self) {
        self.is_preview.set(false);
    }

    fn commit(&self, rest: &Vec<Node>) {
        self.is_sealed.set(true);

        if self.output == 0 {
            return ();
        }

        let next = rest.iter().find(|n| n.id == self.output).unwrap();

        next.commit(rest)
    }
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let total = parse_num::<usize>();
        let fun_values = parse_vec::<usize>();
        let connections = parse_vec::<usize>();

        let modules = (0..total)
            .map(|i| {
                let is_trigger = connections
                    .iter()
                    .enumerate()
                    .filter(|(_, &value)| value == i + 1)
                    .map(|(idx, _)| idx + 1)
                    .collect::<Vec<usize>>()
                    .len()
                    == 0;

                return Node::new(i + 1, fun_values[i], connections[i], is_trigger);
            })
            .collect::<Vec<Node>>();

        let mut total = 0;

        loop {
            let triggers = modules
                .iter()
                .filter(|&x| x.is_trigger && !x.is_sealed.get())
                .collect::<Vec<_>>();

            if triggers.len() == 0 {
                break;
            }

            let mut best_case: Option<(&&Node, usize)> = None;

            for node in &triggers {
                node.set_preview(&modules);

                let next_best_total = modules
                    .iter()
                    .filter(|&x| x.is_trigger && !x.is_preview.get() && !x.is_sealed.get())
                    .map(|n| n.preview_calc(&modules))
                    .sum();

                match best_case {
                    None => best_case = Some((node, next_best_total)),
                    Some((_, bt)) => {
                        if next_best_total > bt {
                            best_case = Some((node, next_best_total));
                        }
                    }
                }

                modules.iter().for_each(|n| n.reset_preview());
            }

            let node = best_case.unwrap().0;
            let result = node.calc(&modules);
            node.commit(&modules);

            total = total + result;
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
