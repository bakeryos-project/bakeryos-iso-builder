use argh::FromArgs;

use crate::mkiso::run_mk_iso;

/// Build ISO
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "build")]
pub struct BuildCmd {}

impl BuildCmd {
    pub fn handle(&self) -> Result<(), String> {
        run_mk_iso()?;

        Ok(())
    }
}
