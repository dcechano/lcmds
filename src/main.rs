use clap::{Parser, Subcommand};
use std::fs::File;
use std::process::exit;
use std::{env, fs};

mod commands;
use crate::commands::{Command, CommandList};

pub const YELLOW: u8 = 11;
pub const BLUE: u8 = 33;
pub const GREEN: u8 = 34;
pub const RED: u8 = 196;

macro_rules! err {
    ($($arg:tt)*) => {{
        use console::style;
        let s = format!($($arg)*);
        println!("\t{}", style(s).color256(RED))
    }};
}

macro_rules! success {
    ($($arg:tt)*) => {{
        use console::style;
        let s = format!($($arg)*);
        println!("\t{}", style(s).color256(GREEN))
    }};
}

#[allow(unused_macros)]
macro_rules! warn {
    ($($arg:tt)*) => {{
        use console::style;
        let s = format!($($arg)*);
        println!("\t{}", style(s).color256(YELLOW))
    }};
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Lcmds {
    #[command(subcommand)]
    commands: Option<Subcommands>,
}
#[derive(Subcommand, Debug)]
enum Subcommands {
    /// List the currently stored commands
    List,
    /// Get a command using the command name:
    /// `get [-c | --cmd ]`
    Get { cmd: String },
    /// Add a command to be referenced later:
    /// `add [-c | --cmd] [-d | --desc]`
    Add(Command),
    /// Try and retrieve a command by matching on part of description.
    // Search { query: String },
    /// Removed a command. Cannot be retrieved later unless added again:
    /// `remove [-c | --cmd ]`
    Remove { cmd: String },
}

const FILE: &str = "commands.toml";

fn main() {
    let args = Lcmds::parse();
    let commands = args.commands.unwrap_or_else(|| {
        eprintln!("No commands passed. Please use --help for help.");
        exit(1);
    });

    let file_path = 'toml: {
        let mut toml = env::current_exe().unwrap();
        toml.pop();
        toml.push(FILE);
        if toml.exists() {
            break 'toml toml;
        }

        let _ = File::create(&toml).map_err(|e| {
            eprintln!(
                "Error creating file {} due to error: {e}",
                toml.to_string_lossy()
            );
            exit(1);
        });
        toml
    };

    let file_str =
        fs::read_to_string(&file_path).expect("Unexpected failure to read from toml file.");
    let mut list = if file_str.is_empty() {
        CommandList::new()
    } else {
        toml::from_str::<CommandList>(&file_str).expect("Unable to parse toml.")
    };
    match commands {
        Subcommands::List => {
            if list.is_empty() {
                err!("No Linux commands stored yet!");
                return;
            }
            print!("Commands:");
            list.iter().for_each(|cmd| println!("{cmd}"));
        }
        Subcommands::Get { cmd } => {
            if list.is_empty() {
                err!("No Linux commands stored yet!");
                return;
            }
            let Some(found_cmd) = list.find(&cmd) else {
                err!("{cmd} has either not been stored or was previously removed");
                return;
            };
            println!("{found_cmd}");
        }
        Subcommands::Add(lcmd) => {
            if list.contains(&lcmd) {
                err!("This command already exists!");
                return;
            }
            success!("{} added.", &lcmd.cmd);
            list.push(lcmd);
            let serialized = toml::to_string(&list).unwrap();
            fs::write(&file_path, serialized).unwrap();
        }
        Subcommands::Remove { cmd } => {
            if list.is_empty() || !list.remove(&cmd) {
                err!("{cmd} has either not been stored or was previously removed");
                return;
            }

            success!("Command {} removed.", &cmd);
            let sered = toml::to_string(&list).expect("Could not serialize command list to toml.");
            if let Err(e) = fs::write(&file_path, sered) {
                eprintln!(
                    "Unable to write new list to {} due to error: {e}",
                    file_path.to_string_lossy()
                );
            }
        }
    }
}
