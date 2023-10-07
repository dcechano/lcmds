use std::fmt::{Display, Formatter};
use clap::Args;
use console::style;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Args, Debug)]
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

impl Eq for Command {}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
            {0}: {2}
            {1}: {3}
            "#,
            style("command").color256(33),
            style("desc").color256(33),
            self.cmd,
            self.desc
        )
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CommandList {
    pub commands: Option<Vec<Command>>,
}

//
// impl IntoIterator for CommandList {
//     type Item = Command;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         if let Some(vec) = self.commands {
//             IntoIterator::into_iter(vec)
//         } else {
//             Vec::new().into_iter()
//         }
//     }
// }
//
// impl CommandList {
//     pub fn iter(&self) -> impl Iterator + '_ {
//         if let Some(ref vec) = self.commands {
//             vec.iter()
//         } else {
//             self.commands.iter()
//         }
//     }
// }
