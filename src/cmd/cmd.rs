use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum, ValueHint};

#[derive(Debug, Parser)]
#[clap(
    about,
    author,
    disable_help_subcommand = true,
    propagate_version = true,
    version
)]
pub enum Cmd {
    Init(Init),
    Set(Set),
}

#[derive(Debug, Parser)]
pub struct Init {}

#[derive(Debug, Parser)]
pub struct Set {}
