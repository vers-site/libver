use diagnostic_quick::QResult;

use crate::{App, AppBuiltin};

impl App {
    pub fn run(&self) -> QResult {
        match &self.cmds {
            Some(AppBuiltin::Fork(cmd)) => cmd.run()?,
            Some(AppBuiltin::New(cmd)) => { cmd.run()? }
            Some(AppBuiltin::External(_)) => {}
            None => {}
        }
        Ok(())
    }
}