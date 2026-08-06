use argh::FromArgs;

/// Cleanup
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "clean")]
pub struct CleanCmd {}
