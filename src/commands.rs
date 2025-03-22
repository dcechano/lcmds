use crate::BLUE;
use clap::Args;
use console::style;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    slice::Iter,
};

#[derive(Serialize, Deserialize, Clone, Args, Debug, Eq)]
pub struct Command {
    /// command to be stored
    #[arg(short, long)]
    pub cmd: String,
    /// description of the command. Put whatever helps you.
    #[arg(short, long)]
    pub desc: String,
}

impl PartialEq<Self> for Command {
    fn eq(&self, other: &Self) -> bool {
        self.cmd.eq(&other.cmd)
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"
        {0}: {2}
        {1}: {3}
        "#,
            style("command").color256(BLUE),
            style("desc").color256(BLUE),
            self.cmd,
            self.desc
        )
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CommandList {
    commands: Vec<Command>,
}

impl CommandList {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn iter(&self) -> Iter<'_, Command> {
        self.commands.iter()
    }
    pub fn push(&mut self, cmd: Command) {
        self.commands.push(cmd);
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn contains(&self, cmd: &Command) -> bool {
        self.commands.contains(cmd)
    }

    pub fn remove(&mut self, cmd: &str) -> bool {
        let Some(i) = self.iter().position(|srch| srch.cmd == cmd) else {
            return false;
        };

        self.commands.remove(i);
        true
    }

    pub fn find(&self, cmd: &str) -> Option<&Command> {
        self.iter().find(|srch| srch.cmd == cmd)
    }
}

impl IntoIterator for CommandList {
    type Item = <Vec<Command> as IntoIterator>::Item;

    type IntoIter = <Vec<Command> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.into_iter()
    }
}
