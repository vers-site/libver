use std::ffi::OsString;

use clap::{Args, Parser, Subcommand};
use diagnostic_quick::QResult;

use self::{fork::CmdFork, new::CmdNew};

pub mod utils;

mod fork;
mod new;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct App {
    #[clap(subcommand)]
    pub cmds: Option<AppBuiltin>,
}

#[derive(Subcommand, Debug)]
pub enum AppBuiltin {
    #[clap(alias = "clone")]
    Fork(Box<CmdFork>),
    #[clap(alias = "init")]
    New(Box<CmdNew>),
    #[clap(external_subcommand, author, version, about, long_about = None)]
    External(Vec<OsString>),
}

#[derive(Args, Debug)]
pub struct CmdShared {
    #[clap(short, long)]
    pub verbose: bool,
    #[clap(short, long)]
    pub quiet: bool,
}

fn main() -> QResult {
    App::parse().run()?;
    Ok(())
}
