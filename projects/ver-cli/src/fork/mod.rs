use std::str::FromStr;

use clap::Parser;
use diagnostic_quick::{error_3rd::Url, QResult};

use crate::CmdShared;

#[doc = include_str!("README.md")]
#[derive(Parser, Debug)]
pub struct CmdFork {
    /// [URL]() address of the repository
    pub url: String,
    /// Local d
    pub local: Option<String>,
    /// Specifies the protocol of the url
    #[clap(long)]
    pub protocol: Option<String>,
    #[clap(flatten)]
    pub other: CmdShared,
}

impl CmdFork {
    pub fn run(&self) -> QResult {
        let url = self.get_url()?;

        println!("download from {:?}", url);
        Ok(())
    }

    fn get_url(&self) -> QResult<Url> {
        let out = if self.url.contains("://") {
            Url::from_str(&self.url)?
        }
        else {
            Url::from_str(&format!("https://vers.site/r/{}", self.url))?
        };
        println!("{:?}", out.host());
        Ok(out)
    }
}
