use std::fs;
use std::io::{self, Write};
use std::process::Command;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    name: String,
    #[structopt(default_value = "run")]
    mode: String,
}

fn main() {
    let args = Cli::from_args();

    // make absolute path to ~/dev/codejam-js
    // make absolute path to ~/dev/codejam-js/example.rs
    // make absolute path to ~/dev/codejam-js/name.rs

    match &args.mode[..] {
        "create" => match fs::copy("", "") {
            Ok(_) => {
                println!("Copied example to {}", args.name);
                // create tests/name.in
            }
            Err(_) => println!("Copy error"),
        },
        "run" => {
            // make absolute path to ~/dev/codejam-js/rls
            let compile = Command::new("rustc")
                .arg(args.name)
                .arg("--out-dir")
                .arg("./rls")
                .output()
                .expect("Failed to Compile");

            io::stdout().write_all(&compile.stdout).unwrap();
            io::stdout().write_all(&compile.stderr).unwrap();

            // make absolute path to ~/dev/codejam-js/rls/name
            // make absolute path to ~/dev/codejam-js/tests/name.in

            // pipe contents of name.in to ./rls/name

            if compile.status.success() {
                let output = Command::new("./rls/name").output().expect("Failed to run!");
                io::stdout().write_all(&output.stdout).unwrap();
                io::stdout().write_all(&output.stderr).unwrap();
            }
        }
        _ => println!("Mode error"),
    }

    println!("Done. Happy Hacking!");
}
