use clap::{Parser, Subcommand};
use diagnostic_quick::QResult;

use self::clone::CmdClone;

pub mod utils;

mod clone;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct App {
    #[clap(subcommand)]
    pub subcommand: Option<AppBuiltin>,
}

impl App {
    pub fn run(&self) -> QResult {
        match &self.subcommand {
            Some(AppBuiltin::Clone(cmd)) => cmd.run(),
            None => Ok(()),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum AppBuiltin {
    Clone(Box<CmdClone>),
}


pub struct CmdShared {
    pub verbose: bool,
    pub quiet: bool,
}

fn main() -> QResult {
    App::parse().run()?;
    Ok(())
}