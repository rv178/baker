use serde_derive::Deserialize;
use std::{
    cmp::Ordering,
    collections::HashMap,
    env, fs,
    io::Write,
    process::{exit, Command, Stdio},
    time::SystemTime,
};

#[macro_export]
macro_rules! printb {
    ($($arg:tt)*) => { println!("\x1b[32mBaker:\x1b[0m {}", format!($($arg)*)); };
}

#[derive(Debug, Deserialize)]
struct Task {
    cmd: String,
    #[serde(default)]
    run: bool,
}

#[derive(Debug, Deserialize)]
struct Recipe {
    build: Task,
    #[serde(default)]
    custom: HashMap<String, Task>,
    #[serde(default)]
    pre: HashMap<String, Task>,
    #[serde(default)]
    env: HashMap<String, String>,
    #[serde(default)]
    debug: bool,
}

impl Recipe {
    fn new() -> Self {
        let path = "recipe.toml";
        let s = fs::read_to_string(path).unwrap_or_else(|_| {
            printb!("Could not find recipe.toml, generating one.");
            let mut f = fs::File::create(path).unwrap();
            f.write_all(b"[build]\ncmd = \"\"").unwrap();
            exit(0);
        });

        toml::from_str(&s).unwrap_or_else(|e| {
            printb!("Error: {}", e);
            exit(1)
        })
    }

    fn execute(&self, name: &str, task: &Task) {
        if task.cmd.is_empty() {
            return;
        }

        if self.debug {
            let start = SystemTime::now();
            if name == "build" {
                printb!("Running build command.");
            } else {
                printb!("Running hook \"{}\".", name);
            }
            self.shell(&task.cmd);
            printb!("Finished in {}ms.", start.elapsed().unwrap().as_millis());
        } else {
            self.shell(&task.cmd);
        }
    }

    fn shell(&self, cmd: &str) {
        if Command::new("sh")
            .args(["-c", cmd])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .is_err()
        {
            exit(1);
        }
    }

    fn set_env_vars(&self) {
        for (key, value) in &self.env {
            printb!("Setting \"{}\" to \"{}\".", key, value);
            env::set_var(key, value);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len().cmp(&1) {
        Ordering::Equal => {
            let recipe = Recipe::new();
            recipe.set_env_vars();

            for (name, task) in &recipe.pre {
                recipe.execute(name, task);
            }

            recipe.execute("build", &recipe.build);

            for (name, task) in &recipe.custom {
                if task.run {
                    recipe.execute(name, task);
                }
            }
        }
        Ordering::Greater => match args[1].as_str() {
            "-h" | "--help" => {
                help();
                exit(0);
            }
            "-v" | "--version" => {
                println!("\x1b[32;1mBaker\x1b[0m ({})", env!("CARGO_PKG_VERSION"));
                exit(0);
            }
            "-c" | "--commands" => {
                print_cmds(&Recipe::new());
                exit(0);
            }
            _ => {
                let recipe = Recipe::new();

                if let Some(task) = recipe.custom.get(args[1].as_str()) {
                    recipe.set_env_vars();
                    recipe.execute(args[1].as_str(), task);
                } else {
                    printb!("Command \"{}\" not found in recipe.toml.", args[1].as_str());
                    exit(1);
                }
            }
        },
        Ordering::Less => {
            exit(1);
        }
    }
}

fn help() {
    println!("\x1b[32m\x1b[1mBaker\x1b[0m");
    println!("  A simple build automation tool.");
    println!();
    println!("\x1b[1mOptions:\x1b[0m ");
    println!("\t-h | --help    \t\t Help command.");
    println!("\t-v | --version \t\t Check version.");
    println!("\t-c | --commands\t\t List commands.");
    println!("\t[command]      \t\t Run a command.");
    println!();
    println!("Link: \x1b[4m\x1b[34mhttps://github.com/rv178/baker/\x1b[0m");
}

fn print_cmds(recipe: &Recipe) {
    println!("\x1b[32mUsage: bake [command]\x1b[0m");
    for name in recipe.custom.keys() {
        println!("\x1b[38;5;8m>\x1b[0m {}", name);
    }
}
