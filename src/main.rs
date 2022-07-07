use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::process::exit;
use std::process::Command;
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

fn main() {
    let args: Vec<String> = env::args().collect();

    let recipe_file =
        fs::read_to_string("recipe.toml").expect("Something went wrong reading the file.");

    let recipe: Recipe =
        toml::from_str(&recipe_file).expect("Something went wrong parsing the file.");

    if recipe.build.cmd.is_empty() {
        println!("[Baker] Build command is empty.");
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
                    exit(1);
                }

                if c.run && args[1] == name {
                    run_cmd(name, cmd);
                }
            }
        }
    }
}

fn run_cmd(name: String, cmd: String) {
    println!("[Baker] Running command: `{}` ({})", cmd, name);
    let start = SystemTime::now();

    let output = Command::new("sh").arg("-c").arg(cmd).output().expect("Failed to execute command");
    let end = SystemTime::now();
    let elapsed = end.duration_since(start);

    println!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    println!("[Baker] {}", output.status);
    println!("[Baker] Took {}ms", elapsed.unwrap_or_default().as_millis());
}
