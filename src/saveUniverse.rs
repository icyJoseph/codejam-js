use std::io;

// define a struct that tracks the cases
struct Tracker {
    cases: u16,
    current: u16,
    problems: Vec<String>,
}

// implement tracking
impl Tracker {
    fn inc(&mut self){  
        self.current = self.current + 1;
    }

    fn get_current(&self) -> u16{
        self.current
    }

    fn set_number_of_cases(&mut self, cases: u16){
        self.cases = cases;
    }

    fn get_number_of_cases(&self) -> u16{
        self.cases
    }

    fn add_problem(&mut self, problem: String) {
        self.problems.push(problem)
    }

    fn get_problems(self) -> Vec<String>{
        self.problems
    }
    
    fn init() -> Tracker {
        Tracker {
            cases:0, 
            current: 0, 
            problems: Vec::new(), 
        }
    }
}

fn apply_dmg(shield: i32, laser: i32) -> [i32; 2] {
    [shield - laser, laser]
}

fn amp_dmg(shield: i32, laser: i32) -> [i32; 2]{
    [shield, laser * 2]
} 

fn swap (commands: &str) -> String {
    // any combination of CS is dangerous
    // it will double the damage and shoot
    // those CS at the end of thr string are the worst!
    // reverse the commands, find SC's on the reversed
    // replace them with CS and return the commands again
    commands.chars().rev().collect::<String>()
            .replacen("SC", "CS", 1).chars().rev().collect::<String>()
}

fn hack (commands: &str, shield: i32, step: i32) -> i32{
    // calculate final shield value by folding over the commands 
    let final_shield:i32 = *commands.chars().fold([shield, 1], |acc, val| {
        match val {
            'C' => amp_dmg(acc[0],acc[1]),
            'S' => apply_dmg(acc[0], acc[1]),
            _ => [-1,-1],
        }
    }).get(0).expect("Problem with final shield");
    // if the final shield is less than 0
    if final_shield < 0 {
        // swap commands
        let swapped_commands:String = swap(&commands);
        // if the swap returns the same command string
        if &swapped_commands == commands {
            // return -1
            return -1;
        }
        // otherwise call hack again, with the new commands
        // the original shield
        // and the step increased by one
        return hack(&swapped_commands, shield, step + 1)
    }
    // if the final shield is more than 0, return the current step
    return step;
}


fn main() {
    // create a Tracker
    let mut tracker = Tracker::init();
    // prepare a heap for starting line
    let mut start_line = String::new();
    // read in the start line
    io::stdin()
        .read_line(&mut start_line)
        .expect("Failed to read start line");
    // parse the start line as u16
    let parsed_start_line: u16 = start_line
                                        .trim()
                                        .parse()
                                        .expect("Failed to parse start line");
    // set the number of cases
    tracker.set_number_of_cases(parsed_start_line);
    // get the number of cases
    let cases = tracker.get_number_of_cases();
    
    loop {
        // ++ the case counter
        tracker.inc();
        // which cases are we solving?
        let current = tracker.get_current();
        
        // if we have gone over the number of cases
        if current > cases {
            break;
        }
        // otherwise, parse this case
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        // add it to the tracker
        tracker.add_problem(line);

    }

    let problems = tracker.get_problems();

   for (index, problem) in problems.iter().enumerate(){
        // case number counts from 1 and up
        let case_number = index + 1;
        let initial_step:i32 = 0;
        // parse the statement
        let statement:Vec<String> = problem
            .split_whitespace()
            .map(|l: &str| l.to_string())
            .collect();

        // match the statement and print something depending on the result of hack
        match statement.as_slice() {
            [shield, commands]  => {
                match hack(&commands, shield.parse::<i32>().unwrap(), initial_step){
                        -1 => println!("Case #{}: IMPOSSIBLE", case_number),
                        res => println!("Case #{}: {}", case_number, res)
                    }
                },
            _   => unreachable!(),
        }
    }
}
