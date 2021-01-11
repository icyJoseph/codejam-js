// https://codingcompetitions.withgoogle.com/codejam/round/00000000000000cb/0000000000007966
use std::io;

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

fn apply_dmg(shield: i32, laser: i32) -> [i32; 2] {
    [shield - laser, laser]
}

fn amp_dmg(shield: i32, laser: i32) -> [i32; 2] {
    [shield, laser * 2]
}

fn swap(commands: &str) -> String {
    // any combination of CS is dangerous
    // it will double the damage and shoot
    // those CS at the end of thr string are the worst!
    // reverse the commands, find SC's on the reversed
    // replace them with CS and return the commands again
    commands
        .chars()
        .rev()
        .collect::<String>()
        .replacen("SC", "CS", 1)
        .chars()
        .rev()
        .collect::<String>()
}

fn hack(commands: &str, shield: i32, step: i32) -> i32 {
    // calculate final shield value by folding over the commands
    let final_shield: i32 = *commands
        .chars()
        .fold([shield, 1], |acc, val| match val {
            'C' => amp_dmg(acc[0], acc[1]),
            'S' => apply_dmg(acc[0], acc[1]),
            _ => [-1, -1],
        })
        .get(0)
        .expect("Problem with final shield");
    // if the final shield is less than 0
    if final_shield < 0 {
        // swap commands
        let swapped_commands: String = swap(&commands);
        // if the swap returns the same command string
        if &swapped_commands == commands {
            // return -1
            return -1;
        }
        // otherwise call hack again, with the new commands
        // the original shield
        // and the step increased by one
        return hack(&swapped_commands, shield, step + 1);
    }
    // if the final shield is more than 0, return the current step
    return step;
}

fn main() -> Res<()> {
    let n = ptc::<i32>();
    for case in 1..=n {
        let next = nxt();
        let mut it = next.split_whitespace();
        let shield = it.next().unwrap().parse::<i32>()?;
        let commands = it.into_iter().collect::<String>();

        match hack(&commands, shield, 0) {
            -1 => println!("Case #{}: IMPOSSIBLE", case),
            res => println!("Case #{}: {}", case, res),
        }
    }
    Ok(())
}
