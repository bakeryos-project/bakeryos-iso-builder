use std::fs;

use ini::configparser::ini::Ini;

use crate::utils::paths::get_all_paths;

pub fn build_pacman_config() -> Result<(), String> {
    let all_paths = get_all_paths()?;
    let pacman_conf_str = all_paths
        .pacman_config_file
        .to_str()
        .ok_or("Invalid pacman config path")?;

    let mut conf = Ini::new();

    conf.load(pacman_conf_str)
        .map_err(|e| format!("Failed to load pacman config: {e}"))?;
    if all_paths.testing_repo_db_file.exists() {
        let abs_path = fs::canonicalize(&all_paths.testing_repo_dir)
            .map_err(|e| format!("Failed to get absolute path: {e}"))?;

        let abs_path_str = abs_path.to_str().ok_or("Invalid absolute path")?;

        let server_url = format!("file://{abs_path_str}");

        conf.set("testing", "SigLevel", Some("Never".to_string()));
        conf.set("testing", "Server", Some(server_url));
    }

    let build_conf_str = all_paths
        .build_pacman_config_file
        .to_str()
        .ok_or("Invalid build pacman config path")?;

    conf.write(build_conf_str)
        .map_err(|e| format!("Failed to save pacman config: {e}"))?;
    Ok(())
}
