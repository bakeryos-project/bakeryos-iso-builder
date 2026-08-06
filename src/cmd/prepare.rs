use crate::{
    helpers::{
        exec::exec_command,
        repo::{build_testing_repo, sync_testing_repo},
    },
    utils::paths::{get_current_working_dir, init_file_system},
};
use argh::FromArgs;
use log::{info, warn};

/// Prepare environment
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "prepare")]
pub struct PrepareCmd {}

const NEEDED_PKGS: &[&'static str] = &[
    "base-devel",
    "squashfs-tools",
    "dosfstools",
    "mtools",
    "arch-install-scripts",
    "xorriso",
];

impl PrepareCmd {
    pub fn handle(&self) -> Result<(), String> {
        let current_wokring_dir = get_current_working_dir()?;

        info!("Updating ...");
        exec_command("sudo", ["pacman", "-Syu"], &current_wokring_dir)?;
        let mut args = vec!["pacman", "-S", "--needed"];
        args.extend(NEEDED_PKGS);
        exec_command("sudo", args, &current_wokring_dir)?;

        info!("Setting up file system ...");
        init_file_system()?;

        info!("Sync testing repository");
        sync_testing_repo()?;

        info!("Build testing database");
        build_testing_repo()?;
        Ok(())
    }
}
