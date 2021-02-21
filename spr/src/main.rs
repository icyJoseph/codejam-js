extern crate confy;

#[macro_use]
extern crate serde_derive;

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

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    path: String,
    src_dir: String,
    out_dir: String,
    test_dir: String,
    test_ext: String,
    file_ext: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            path: "/home/joseph/dev/codejam-js".to_string(),
            src_dir: "src".to_string(),
            out_dir: "rls".to_string(),
            test_dir: "tests".to_string(),
            test_ext: "in".to_string(),
            file_ext: "rs".to_string(),
        }
    }
}

fn main() {
    let args = Cli::from_args();

    let cfg = match confy::load::<Config>("spr") {
        Ok(config) => config,
        Err(_) => Config::default(),
    };

    let filename = format!(
        "{}/{}/{}.{}",
        cfg.path, cfg.src_dir, args.name, cfg.file_ext
    );

    let test_path = format!(
        "{}/{}/{}.{}",
        cfg.path, cfg.test_dir, args.name, cfg.test_ext
    );

    match &args.mode[..] {
        "create" => {
            let target_file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(filename);

            let example = format!(
                "{}/{}/{}.{}",
                cfg.path, cfg.src_dir, "example", cfg.file_ext
            );

            let mut example_file = fs::File::open(example).unwrap();

            match target_file {
                Ok(mut file) => match io::copy(&mut example_file, &mut file) {
                    Ok(_) => {
                        println!("Created: {}", args.name);

                        match fs::File::create(test_path) {
                            // TODO: paste test file contents to stdin
                            Ok(_) => println!("Created test file for: {}", args.name),
                            Err(why) => panic!("Failed to create test file.\n{}", why),
                        }
                    }
                    Err(why) => panic!("Failed to create:\n{}", why),
                },
                Err(why) => panic!("Already exists!\n{}", why),
            }
        }
        "run" => {
            let output_path = format!("{}/{}", cfg.path, cfg.out_dir);

            let compile = Command::new("rustc")
                .arg(filename)
                .arg("--out-dir")
                .arg(output_path)
                .output()
                .expect("[Failed to compile]");

            io::stdout().write_all(&compile.stdout).unwrap();
            io::stdout().write_all(&compile.stderr).unwrap();

            if compile.status.success() {
                let rls_path = format!("{}/{}/{}", cfg.path, cfg.out_dir, args.name);

                let mut process = match Command::new(rls_path)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                {
                    Err(why) => panic!("[Failed to run]:\n{}", why),
                    Ok(process) => process,
                };

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
        _ => panic!("Supported modes: `run` or `create`"),
    }

    println!("Done. Happy Hacking!");
}
