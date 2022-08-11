use ::std::{
    cmp::Ordering,
    collections::HashMap,
    env,
    fs::{read_to_string, File},
    io::{ErrorKind, Write},
    process::{exit, Command, Stdio},
    time::SystemTime,
};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Recipe {
    build: Build,
    custom: Option<HashMap<String, Custom>>,
    pre: Option<HashMap<String, Pre>>,
    env: Option<HashMap<String, String>>,
    debug: Option<bool>,
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

        let recipe: Recipe = match toml::from_str(&recipe_str) {
            Ok(r) => r,
            Err(e) => {
                printb!("Error: {}", e);
                exit(1);
            }
        };
        Some(recipe)
    }
}

trait Runnable {
    fn execute(&self, name: String, debug: bool);
}

#[derive(Debug, Deserialize)]
struct Build {
    cmd: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Custom {
    cmd: String,
    run: bool,
}

#[derive(Debug, Deserialize, Clone)]
struct Pre {
    cmd: String,
}

impl Runnable for Build {
    fn execute(&self, name: String, debug: bool) {
        if self.cmd.is_empty() {
            printb!("Build command is empty.");
            exit(1);
        }

        run_cmd(name, self.cmd.to_string(), debug);
    }
}

impl Runnable for Custom {
    fn execute(&self, name: String, debug: bool) {
        if self.cmd.is_empty() {
            printb!("Custom command is empty.");
            exit(1);
        }

        run_cmd(name, self.cmd.to_string(), debug);
    }
}

impl Runnable for Pre {
    fn execute(&self, name: String, debug: bool) {
        if self.cmd.is_empty() {
            printb!("Pre command is empty.");
            exit(1);
        }

        run_cmd(name, self.cmd.to_string(), debug);
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
        print_cmds();
        exit(1);
    }

    let recipe: Recipe = Recipe::new().unwrap();

    let debug = recipe.debug.unwrap_or(false);

    if recipe.env.is_some() {
        let env = recipe.env.unwrap();
        for (key, value) in env {
            if debug {
                printb!("Setting \"{}\" to \"{}\".", key, value);
            }
            env::set_var(key, value);
        }
    }

    if args.len() == 1 {
        if recipe.pre.is_some() {
            let pre = recipe.pre.unwrap();
            for (name, p) in pre {
                p.execute(name, debug);
            }
        }
        recipe.build.execute("build".to_string(), debug);
    }

    if recipe.custom.is_some() {
        let custom = recipe.custom.unwrap();
        for (name, c) in custom {
            match args.len().cmp(&1) {
                Ordering::Equal => {
                    if c.run {
                        c.execute(name, debug);
                    }
                }
                Ordering::Less => {
                    exit(1);
                }
                Ordering::Greater => {
                    if args[1] == name {
                        c.execute(name, debug);
                    }
                }
            }
        }
    }
}

fn run_cmd(name: String, cmd: String, debug: bool) {
    if debug {
        let start = SystemTime::now();

        match name.as_str() {
            "build" => {
                println!();
                printb!("Running build command.");
            }
            _ => {
                println!();
                printb!("Running hook \"{}\".", name);
            }
        }
        run(cmd, debug);
        printb!("Finished in {}ms.", start.elapsed().unwrap().as_millis());
    } else {
        run(cmd, debug);
    }

    fn run(cmd: String, debug: bool) {
        let cmd = cmd.split("&&").collect::<Vec<&str>>();

        for command in cmd {
            let split: Vec<&str> = command.split_whitespace().collect();
            let mut cmd_arr: Vec<String> = Vec::new();

            for item in split {
                let parsed = shellexpand::full(&item).unwrap();
                cmd_arr.push(parsed.to_string());
            }

            if debug {
                printb!("Command: {:?}", cmd_arr);
            }

            match Command::new(&cmd_arr[0])
                .args(&cmd_arr[1..])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
            {
                Ok(_) => {}
                Err(e) => {
                    printb!("Error: {}", e);
                    exit(1);
                }
            }
        }
    }
}

fn version() {
    println!("\x1b[32m\x1b[1mBaker\x1b[0m ({})", env!("CARGO_PKG_VERSION"));
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

fn print_cmds() {
    let recipe: Recipe = Recipe::new().unwrap();
    println!("\x1b[32mUsage: bake [command]\x1b[0m");
    if recipe.custom.is_some() {
        let custom = recipe.custom.unwrap();

        for (name, _custom) in custom {
            println!("\x1b[38;5;8m>\x1b[0m {}", name);
        }
    }
}
