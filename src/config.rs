use crate::{debug, error, utils};
use serde_derive::Deserialize;
use std::{
    collections::HashMap,
    env,
    fs::{File, read_to_string},
    io::{ErrorKind, Write},
    process::{Command, Stdio, exit},
    time::SystemTime,
};

#[derive(Debug, Deserialize)]
pub struct Task {
    cmd: String,
    #[serde(default)]
    pub run: bool,
}

impl Task {
    pub fn run(&self) {
        if Command::new("sh")
            .args(["-c", self.cmd.as_str()])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .is_err()
        {
            exit(1);
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Recipe {
    pub build: Task,
    #[serde(default)]
    pub custom: HashMap<String, Task>,
    #[serde(default)]
    pub pre: HashMap<String, Task>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    debug: bool,
}

pub enum Type<'a> {
    Build,
    Custom(&'a str),
    Pre(&'a str),
}

impl Recipe {
    pub fn new() -> Self {
        let mut rec_str = String::new();
        match read_to_string("recipe.toml") {
            Ok(s) => rec_str.push_str(&s),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    error!("could not find recipe.toml, generating one.");
                    let mut f = File::create("recipe.toml").unwrap();
                    f.write_all(b"[build]\ncmd = \"\"").unwrap();
                    exit(0);
                }
                _ => {
                    error!("{}", e);
                }
            },
        }

        let recipe: Recipe = match toml::from_str(&rec_str) {
            Ok(r) => r,
            Err(e) => {
                error!("{}", e);
                exit(1);
            }
        };
        recipe
    }

    pub fn execute(&self, ty: Type, task: &Task) {
        if task.cmd.is_empty() {
            return;
        }

        if self.debug {
            let start = SystemTime::now();
            match ty {
                Type::Build => {
                    debug!("running {}build command{}.", utils::GREEN, utils::RESET);
                }
                Type::Custom(s) => {
                    debug!(
                        "running {}custom{} hook \"{}{}{}\".",
                        utils::GREEN,
                        utils::RESET,
                        utils::YELLOW,
                        s,
                        utils::RESET
                    );
                }
                Type::Pre(s) => {
                    debug!(
                        "running {}pre{} hook \"{}{}{}\".",
                        utils::GREEN,
                        utils::RESET,
                        utils::YELLOW,
                        s,
                        utils::RESET
                    );
                }
            }
            task.run();
            debug!(
                "finished in {}{}ms{}.",
                utils::YELLOW,
                start.elapsed().unwrap().as_millis(),
                utils::RESET
            );
        } else {
            task.run();
        }
    }

    pub fn set_env_vars(&self) {
        for (key, value) in &self.env {
            if self.debug {
                debug!(
                    "setting \"{}{}{}\" to \"{}{}{}\".",
                    utils::YELLOW,
                    key,
                    utils::RESET,
                    utils::YELLOW,
                    value,
                    utils::RESET
                );
            }
            unsafe {
                env::set_var(key, value);
            }
        }
    }

    pub fn print_cmds(&self) {
        println!("{}Usage: bake [command]{}", utils::GREEN, utils::RESET);
        for name in self.custom.keys() {
            if self.custom[name].run {
                println!("{}> {}{}{}", utils::BLACK, utils::BLUE, name, utils::RESET);
            } else {
                println!("{}>{} {}", utils::BLACK, utils::RESET, name);
            }
        }
        println!("{}Configured pre hooks:{}", utils::GREEN, utils::RESET);
        for name in self.pre.keys() {
            println!("{}>{} {}", utils::BLACK, utils::RESET, name);
        }
    }
}
