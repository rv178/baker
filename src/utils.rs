#[macro_export]
macro_rules! log {
    ($colour:expr, $label:expr, $($arg:tt)*) => ({
        println!("{}{}:\x1b[0m {}", $colour, $label, format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ($crate::log!("\x1b[32m", "info", $($arg)*));
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ($crate::log!("\x1b[31m", "error", $($arg)*));
}

pub const RESET: &str = "\x1b[0m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const BLACK: &str = "\x1b[38;5;8m";
pub const BOLD: &str = "\x1b[1m";
pub const UNDERLINE: &str = "\x1b[4m";

pub fn help() {
    let help_msg = format!(
        "{GREEN}{BOLD}baker {RESET} {version}
    A simple build automation tool.

{YELLOW}USAGE:{RESET}
    bake {GREEN}[OPTIONS]{RESET}

{YELLOW}OPTIONS:{RESET}
    {GREEN}-h, --help{RESET}
        Show this help message.
    {GREEN}-c, --commands{RESET}
        List all commands.
    {GREEN}[command]{RESET}
        Run a command.

Link: {UNDERLINE}{BLUE}https://github.com/rv178/baker{RESET}",
        version = env!("CARGO_PKG_VERSION"),
    );
    println!("{}", help_msg);
}
