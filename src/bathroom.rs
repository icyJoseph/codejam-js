use std::io;

use std::collections::{HashSet, HashMap};
    
type Res<T> = Result<T, Box<dyn std::error::Error>>;

fn nxt() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        _ => panic!("Error reading line"),
    }
}

fn ptc<T: std::str::FromStr>() -> T {
    match nxt().trim().parse::<T>() {
        Ok(n) => n,
        _ => panic!("Error parsing"),
    }
}

fn split(n: u64) -> (u64, u64) {
    if n % 2 == 0 {
        return (n / 2 - 1, n / 2);
    }
    return (n / 2, n / 2);
}

fn max(s:&HashSet<u64>)-> u64{
    match s.iter().max() {
        Some(&n)=> n,
        _=>panic!("Empty")
    }
}

fn main() -> Res<()> {
    let n = ptc::<i32>();

    for case in 1..=n {
        let entry: Vec<u64> = nxt()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let s = entry[0];
        let k = entry[1];
    
        let mut set: HashSet<u64> = HashSet::new();
        let mut pres:HashMap<u64, u64> = HashMap::new();

        let mut count = 0;

        set.insert(s);
        pres.insert(s,1);

        let mut l = 0;
        let mut r = 0;

        loop {
            if count >= k {
                break;
            }

            let next = max(&set);

            let qty = *pres.get(&next).unwrap();

            count = count + qty;

            let sp = split(next);

            l = sp.0;
            r = sp.1;
            
            set.remove(&next);
            set.insert(sp.0);
            set.insert(sp.1);
            
            pres.remove(&next);
            let a = pres.entry(sp.0).or_insert(0);
            *a += qty;
            let b = pres.entry(sp.1).or_insert(0);
            *b += qty;

        }


        println!(
            "Case #{}: {} {}",
            case,
            std::cmp::max(l, r),
            std::cmp::min(l, r)
        );
    }
    
    Ok(())
}
