mod commands;
use crate::commands::{Command, CommandList};
use clap::{Parser, Subcommand};
use std::{env, fs};
use std::fs::File;
use std::process::{exit};
use std::str::FromStr;

macro_rules! err {
    ($($arg:tt)*) => {{
        use console::style;
        let s = format!($($arg)*);
        println!("\t{}", style(s).color256(196))
    }};
}

macro_rules! success {
    ($($arg:tt)*) => {{
        use console::style;
        let s = format!($($arg)*);
        println!("\t{}", style(s).color256(34))
    }};
}

macro_rules! warn {
    ($($arg:tt)*) => {{
        use console::style;
        let s = format!($($arg)*);
        println!("\t{}", style(s).color256(11))
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
    // /// Try and retrieve a command by matching on part of description.
    // GetDsc { cmd: String },
    /// Removed a command. Cannot be retrieved later unless added again:
    /// `remove [-c | --cmd ]`
    Remove { cmd: String },
}

const FILE: &str = "commands.toml";

fn main() {
    let args = Lcmds::parse();
    let commands = args.commands.unwrap_or_else(|| {
        err!("No commands passed. Please use --help for help.");
        exit(1);
    });

    let file_path = {
        let mut toml = env::current_exe().unwrap();
        toml.pop();
        toml.push(FILE);
        if toml.exists() {
            toml
        } else {
            let _ = File::create(&toml).unwrap();
            toml
        }
    };

    match commands {
        Subcommands::List => {
            let file_str = &fs::read_to_string(file_path).unwrap();
            let list = toml::from_str::<CommandList>(file_str).unwrap();
            match list.commands {
                None => {
                    err!("No Linux commands stored yet!");
                }
                Some(cmds) => {
                    print!("Commands:");
                    for cmd in cmds {
                        println!("{}", cmd);
                    }
                }
            }
        }
        Subcommands::Get { cmd } => {
            let file_str = &fs::read_to_string(file_path).unwrap();
            let list = toml::from_str::<CommandList>(file_str).unwrap();
            let mut ind = None;
            match list.commands {
                None => {
                    err!("No Linux commands stored yet!");
                }
                Some(cmds) => {
                    for (i, srch) in cmds.iter().enumerate() {
                        if srch.cmd.eq(&cmd) {
                            ind = Some(i);
                        }
                    }
                    if let Some(i) = ind {
                        println!("{}", cmds[i]);
                    } else {
                        err!("{cmd} has either not been stored or was previously removed");
                    }
                }
            }
        }
        Subcommands::Add(lcmd) => {
            let file_str = &fs::read_to_string(&file_path).unwrap();
            let mut list = toml::from_str::<CommandList>(file_str).unwrap();
            match list.commands {
                None => {
                    list.commands = Some(vec![lcmd]);
                }
                Some(ref mut vec) => {
                    if vec.contains(&lcmd) {
                        err!("This command already exists!");
                    } else {
                        success!("{} added.", &lcmd.cmd);
                        vec.push(lcmd);
                    }
                }
            }
            let reser = toml::to_string(&list).unwrap();
            fs::write(&file_path, reser).unwrap();
        }
        // Subcommands::GetDsc { cmd } => {
        //     println!("You asked for this {cmd} description!");
        // }
        Subcommands::Remove { cmd } => {
            let file_str = &fs::read_to_string(&file_path).unwrap();
            let mut list = toml::from_str::<CommandList>(file_str).unwrap();
            match list.commands {
                None => {
                    err!("{cmd} has either not been stored or was previously removed");
                }
                Some(ref mut cmds) => {
                    let mut ind = None;
                    for (i, srch) in cmds.iter_mut().enumerate() {
                        if srch.cmd.eq(&cmd) {
                            ind = Some(i);
                        }
                    }
                    if let Some(i) = ind {
                        cmds.remove(i);
                        success!("Command {} removed.", &cmd)
                    } else {
                        err!("{cmd} has either not been stored or was previously removed");
                    }
                }
            }

            let sered = toml::to_string(&list).unwrap();
            fs::write(&file_path, sered).unwrap();
        }
    }
}
