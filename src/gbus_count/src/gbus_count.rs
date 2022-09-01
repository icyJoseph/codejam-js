use std::io;

fn count_cities(services: &Vec<(usize, usize)>, city: usize) -> usize {
    services
        .iter()
        .filter(|(start, end)| city >= *start && city <= *end)
        .count()
}

fn main() -> Res<()> {
    let n = parse_num::<u32>();
    for case in 1..=n {
        let _ = parse_num::<usize>();

        let services_vec = parse_vec::<usize>();

        let cities_len = parse_num::<usize>();

        let mut cities: Vec<usize> = vec![];

        for _ in 0..cities_len {
            cities.push(parse_num::<usize>());
        }

        let mut services: Vec<(usize, usize)> = vec![];

        for (index, city) in services_vec.iter().enumerate() {
            if index % 2 == 0 {
                services.push((*city, services_vec[index + 1]));
            }
        }

        let bus_total = cities
            .iter()
            .map(|&city| count_cities(&services, city))
            .collect::<Vec<usize>>();

        println!("Case #{}: {}", case, string_vec(&bus_total, " "));

        nxt();
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
