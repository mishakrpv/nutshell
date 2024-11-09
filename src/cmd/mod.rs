mod cmd;
mod init;
mod set;

use anyhow::Result;

pub use crate::cmd::cmd::*;

pub trait Run {
    fn run(&self) -> Result<()>;
}

impl Run for Cmd {
    fn run(&self) -> Result<()> {
        match self {
            Cmd::Init(cmd) => cmd.run(),
            Cmd::Set(cmd) => cmd.run(),
        }
    }
}
