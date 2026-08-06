use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn exec_command<I, S>(program: &str, args: I, dir: &PathBuf) -> Result<(), String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let mut cmd = Command::new(program);
    for a in args {
        cmd.arg(a);
    }

    let status = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(dir)
        .status()
        .map_err(|e| e.to_string())?;

    if !status.success() {
        return Err(format!(
            "Command failed with exit code: {:?}",
            status.code()
        ));
    }

    Ok(())
}
