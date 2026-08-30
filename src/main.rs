use crate::config::{Recipe, Type};
use std::{cmp, env, process::exit};

mod config;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len().cmp(&1) {
        cmp::Ordering::Equal => {
            let recipe: Recipe = Recipe::new();
            recipe.set_env_vars();

            for (name, task) in &recipe.pre {
                recipe.execute(Type::Pre(&name), task);
            }

            recipe.execute(Type::Build, &recipe.build);

            for (name, task) in &recipe.custom {
                if task.run {
                    recipe.execute(Type::Custom(&name), task);
                }
            }
        }
        cmp::Ordering::Greater => match args[1].as_str() {
            "-h" | "--help" => {
                utils::help();
            }
            "-c" | "--commands" => {
                let recipe = Recipe::new();
                recipe.print_cmds();
            }
            _ => {
                let recipe = Recipe::new();

                if let Some(task) = recipe.custom.get(&args[1]) {
                    recipe.set_env_vars();
                    recipe.execute(Type::Custom(&args[1]), task);
                } else {
                    error!("command \"{}\" not found in recipe.toml.", args[1].as_str());
                    exit(1);
                }
            }
        },
        cmp::Ordering::Less => {
            exit(1);
        }
    }
}
