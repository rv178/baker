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

impl Recipe {
    fn new() -> Option<Self> {
        if let Err(e) = File::open("recipe.toml") {
            if e.kind() == ErrorKind::NotFound {
                printb!("Could not find a recipe.toml, generating one.");
                let mut file = File::create("recipe.toml").unwrap();
                file.write_all(b"[build]\ncmd = \"\"").unwrap();
                if env::current_dir().is_ok() {
                    printb!("Generated file in {:?}", env::current_dir().unwrap());
                } else {
                    printb!("Generated file.");
                }
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

        let recipe: Recipe;

        match toml::from_str(&recipe_str) {
            Ok(r) => recipe = r,
            Err(e) => {
                printb!("Error: {}", e);
                exit(1);
            }
        }
        Some(recipe)
    }
}

trait Runnable {
    fn execute(&self);
}

#[derive(Debug, Deserialize)]
struct Build {
    cmd: String,
}

impl Runnable for Build {
    fn execute(&self) {
        if self.cmd.is_empty() {
            printb!("Build command is empty.");
            exit(1);
        }

        run_cmd("build".to_string(), self.cmd.to_string());
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Custom {
    name: String,
    cmd: String,
    run: bool,
}

impl Runnable for Custom {
    fn execute(&self) {
        if self.cmd.is_empty() {
            printb!("Custom command `{}` is empty.", self.cmd);
            exit(1);
        }

        run_cmd(self.name.to_string(), self.cmd.to_string());
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Pre {
    name: String,
    cmd: String,
}

impl Runnable for Pre {
    fn execute(&self) {
        if self.cmd.is_empty() {
            printb!("Pre command `{}` is empty.", self.cmd);
            exit(1);
        }

        run_cmd(self.name.to_string(), self.cmd.to_string());
    }
}

#[macro_export]
macro_rules! printb {
    ($($arg:tt)*) => {
        println!("\x1b[32mBaker:\x1b[0m {}", format!($($arg)*));
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1].eq("-v") | args[1].eq("--version") {
        version();
        exit(1);
    }

    if args.len() > 1 && args[1].eq("-h") | args[1].eq("--help") {
        help();
        exit(1);
    }

    if args.len() > 1 && args[1].eq("-c") | args[1].eq("--commands") {
        print_cmds(args[0].to_string());
        exit(1);
    }

    let recipe: Recipe = Recipe::new().unwrap();

    if args.len() == 1 {
        if recipe.pre.is_some() {
            let pre = recipe.pre.unwrap();

            for p in pre {
                p.execute()
            }
        }
        recipe.build.execute();
    }

    if recipe.custom.is_some() {
        let custom = recipe.custom.unwrap();

        for c in custom {
            if c.run && args.len() == 1 {
                c.execute();
                exit(0);
            }

            if args.len() > 1 {
                if args[1] == c.name {
                    c.execute();
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

fn version() {
    println!("\x1b[32m\x1b[1mBaker\x1b[0m ({})", env!("CARGO_PKG_VERSION"));
}

fn help() {
    println!("\x1b[32m\x1b[1mBaker\x1b[0m");
    println!("  A simple build automation tool.");
    print!("\n");
    println!("\x1b[1mOptions:\x1b[0m ");
    println!("\t-h | --help    \t\t Help command.");
    println!("\t-v | --version \t\t Check version.");
    println!("\t-c | --commands\t\t List commands.");
    println!("\t[command]      \t\t Run a command.");
    print!("\n");
    println!("Link: \x1b[4m\x1b[34mhttps://github.com/rv178/baker/\x1b[0m");
}

fn print_cmds(cmd: String) {
    let recipe: Recipe = Recipe::new().unwrap();
    println!("Commands: ");
    println!("\t{}", cmd);
    if recipe.custom.is_some() {
        let custom = recipe.custom.unwrap();

        for c in custom {
            println!("\t{} \x1b[33m{}\x1b[0m", cmd, c.name);
        }
    }
}
