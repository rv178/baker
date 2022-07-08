use serde_derive::Deserialize;
use std::env;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::process::{exit, Command, Stdio};
use std::time::SystemTime;

#[derive(Debug, Deserialize)]
struct Recipe {
    build: Build,
    custom: Option<Vec<Custom>>,
    pre: Option<Vec<Pre>>,
}

#[derive(Debug, Deserialize)]
struct Build {
    cmd: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Custom {
    name: String,
    cmd: String,
    run: bool,
}

#[derive(Debug, Deserialize, Clone)]
struct Pre {
    name: String,
    cmd: String,
}

#[macro_export]
macro_rules! printb {
    ($($arg:tt)*) => {
        println!("\x1b[32mBaker:\x1b[0m {}", format!($($arg)*));
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open("recipe.toml");

    if let Err(e) = file {
        if e.kind() == ErrorKind::NotFound {
            printb!("Could not find a recipe.toml, generating one.");
            let mut file = File::create("recipe.toml").unwrap();
            file.write_all(b"[build]\ncmd = \"\"").unwrap();
            exit(0);
        } else {
            printb!("Error: {}", e);
        }
    }

    let mut recipe_str = String::new();

    match read_to_string("recipe.toml") {
        Ok(s) => recipe_str.push_str(&s),
        Err(e) => {
            printb!("Error: {}", e);
            exit(1);
        }
    }

    let recipe: Recipe = toml::from_str(&recipe_str).expect("Failed to parse recipe.toml");

    if recipe.build.cmd.is_empty() {
        printb!("Build command is empty.");
        exit(1);
    }

    if args.len() == 1 {
        if recipe.pre.is_some() {
            let pre = recipe.pre.unwrap();

            for p in pre {
                run_cmd(p.name, p.cmd)
            }
        }
        run_cmd("build".to_string(), recipe.build.cmd);
    }

    if recipe.custom.is_some() {
        let custom = recipe.custom.unwrap();

        for c in custom {
            let cmd = c.cmd.clone();
            let name = c.name.clone();

            if c.run && args.len() == 1 {
                run_cmd(c.name, c.cmd);
            }

            if args.len() > 1 {
                if args[1] == name {
                    run_cmd(name, cmd);
                    exit(0);
                }

                if c.run && args[1] == name {
                    run_cmd(name, cmd);
                }
            }
        }
    }
}

fn run_cmd(name: String, cmd: String) {
    printb!("Running command: `{}` ({})", cmd, name);
    print!("\n");
    let start = SystemTime::now();

    match Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
    {
        Ok(_) => {}
        Err(e) => {
            printb!("Failed to execute command. Error: `{}` ({})", e, name);
        }
    }
    let end = SystemTime::now();
    let elapsed = end.duration_since(start);

    print!("\n");
    printb!("Took {}ms", elapsed.unwrap_or_default().as_millis());
}
