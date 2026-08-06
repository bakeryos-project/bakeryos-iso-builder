use argh::FromArgs;

use crate::cmd::{build::BuildCmd, clean::CleanCmd, prepare::PrepareCmd};

/// BakeryOS ISO Builder
#[derive(FromArgs, Debug)]
pub struct Root {
    #[argh(subcommand)]
    pub nested: SubCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCommand {
    Build(BuildCmd),
    Clean(CleanCmd),
    Prepare(PrepareCmd),
}

impl Root {
    pub fn handle(&self) -> Result<(), String> {
        match &self.nested {
            SubCommand::Prepare(cmd) => cmd.handle()?,
            SubCommand::Build(cmd) => cmd.handle()?,

            _ => {}
        }

        Ok(())
    }
}
