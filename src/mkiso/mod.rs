use std::fs;

use crate::helpers::exec::exec_command;
use crate::helpers::pacman::build_pacman_config;
use crate::utils::paths::get_all_paths;

const BASH_SCRIPT: &str = include_str!("run.sh");

pub fn run_mk_iso() -> Result<(), String> {
    let all_paths = get_all_paths()?;
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join(format!("bakeryos-run-{}.sh", std::process::id()));

    fs::write(&temp_file_path, BASH_SCRIPT).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&temp_file_path)
            .map_err(|e| e.to_string())?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&temp_file_path, perms).map_err(|e| e.to_string())?;
    }
    struct Guard(std::path::PathBuf);
    impl Drop for Guard {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.0);
        }
    }
    let _guard = Guard(temp_file_path.clone());

    build_pacman_config()?;

    let temp_file_str = temp_file_path
        .to_str()
        .ok_or("Invalid path for temp file")?;
    let config_file_str = all_paths
        .build_pacman_config_file
        .to_str()
        .ok_or("Invaild path")?;
    let work_dir_str = all_paths.work_dir.to_str().ok_or("Invalid path")?;
    let out_dir_str = all_paths.out_dir.to_str().ok_or("Invalid path")?;

    let script_args = vec![
        temp_file_str,
        "-v",
        "-C",
        config_file_str,
        "-w",
        work_dir_str,
        "-o",
        out_dir_str,
        ".",
    ];

    let mut cmd_args = vec!["bash"];
    cmd_args.extend(script_args);

    exec_command("sudo", &cmd_args, &all_paths.current_working_dir)?;

    Ok(())
}
