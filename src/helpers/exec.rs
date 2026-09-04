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

    let output = cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(dir)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        return Err(format!(
            "Command '{}' failed with exit code: {:?}\n--- STDOUT ---\n{}\n--- STDERR ---\n{}",
            program,
            output.status.code(),
            stdout,
            stderr
        ));
    }

    Ok(())
}
