extern crate confy;

use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    name: String,
    #[structopt(short = "m", default_value = "run")]
    mode: String,
}

fn main() {
    let args = Cli::from_args();

    let problem_name = args.name;

    match &args.mode[..] {
        "run" => {
            let compile = Command::new("cargo")
                .arg("run")
                .arg(&problem_name)
                .arg("--release")
                .output()
                .expect("[Failed to compile]");

            io::stdout().write_all(&compile.stdout).unwrap();
            io::stdout().write_all(&compile.stderr).unwrap();

            if compile.status.success() {
                let rls_path = format!("./{}/{}/{}", "target", "release", problem_name);

                let mut process = match Command::new(rls_path)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                {
                    Err(why) => panic!("[Failed to run]:\n{}", why),
                    Ok(process) => process,
                };

                let test_path = format!(
                    "./{}/{}/{}/{}.{}",
                    "src", problem_name, "src", problem_name, "in"
                );

                let mut file = fs::File::open(test_path).expect("[Failed to Open File]");

                io::copy(&mut file, process.stdin.as_mut().unwrap())
                    .expect("[Failed to stream tests to compiled binary]");

                let output = process.wait_with_output().unwrap();

                if output.status.success() {
                    println!("\n{}", String::from_utf8(output.stdout).unwrap());
                } else {
                    println!("\n{}", String::from_utf8(output.stderr).unwrap());
                }
            }
        }
        _ => panic!("Supported modes: `run`"),
    }

    println!("Done. Happy Hacking!");
}
