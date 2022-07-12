use ::std::{
    collections::HashMap,
    env,
    fmt::Write as _,
    fs::{read_to_string, File},
    io::{ErrorKind, Write},
    process::{exit, Command, Stdio},
    time::SystemTime,
};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Recipe {
    build: Build,
    custom: Option<Vec<Custom>>,
    pre: Option<Vec<Pre>>,
    env: Option<HashMap<String, String>>,
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
            printb!("Custom command \"{}\" is empty.", self.cmd);
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
            printb!("Pre command \"{}\" is empty.", self.cmd);
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

    if recipe.env.is_some() {
        let env = recipe.env.unwrap();
        for (key, value) in env {
            printb!("Setting {} to \"{}\"", key, value);
            env::set_var(key, value);
        }
        println!();
    }

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

            if args.len() > 1 && args[1] == c.name {
                c.execute();
            }
        }
    }
}

fn run_cmd(name: String, cmd: String) {
    printb!("Running \"{}\"", name);
    println!();
    let start = SystemTime::now();

    let cmd = cmd.split("&&").collect::<Vec<&str>>();
    for c in cmd {
        let cmds: Vec<&str> = c.split_whitespace().collect();
        let mut cmd_arr = Vec::new();

        for cmd in cmds {
            if cmd.contains('$') {
                // if cmd contains $ (like ./bin/$BIN_NAME) then we split the string at the $
                // that gives us "./bin" and "BIN_NAME"
                // we replace "BIN_NAME" with value of env var of same name
                let cmd_split = cmd.split('$');
                let mut cmd_str = String::new();
                for s in cmd_split {
                    // for directory paths (example: ./path/$BIN_NAME)
                    if s.contains('/') {
                        let s = s.strip_suffix('/').expect("Failed to strip / suffix");
                        if env::var(s).is_ok() {
                            write!(cmd_str, "{}/", env::var(s).unwrap())
                                .expect("Failed to write to cmd_str");
                        } else {
                            write!(cmd_str, "{}/", s).expect("Failed to write to cmd_str");
                        }
                    } else if env::var(s).is_ok() {
                        cmd_str.push_str(&env::var(s).unwrap());
                    } else {
                        cmd_str.push_str(s);
                    }
                }
                cmd_arr.push(cmd_str);
            } else {
                // if none of these conditions are met then we just push the cmd to the array
                cmd_arr.push(cmd.to_string());
            }
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
    let end = SystemTime::now();
    let elapsed = end.duration_since(start);

    println!();
    printb!("Took {}ms", elapsed.unwrap_or_default().as_millis());
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
