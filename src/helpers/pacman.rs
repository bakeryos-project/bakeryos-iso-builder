use crate::utils::paths::get_all_paths;
use std::fs;

pub fn build_pacman_config() -> Result<(), String> {
    let all_paths = get_all_paths()?;

    let content = fs::read_to_string(&all_paths.pacman_config_file)
        .map_err(|e| format!("Failed to read pacman config: {e}"))?;

    let mut new_content = content;

    if all_paths.testing_repo_db_file.exists() {
        let abs_path = fs::canonicalize(&all_paths.testing_repo_dir)
            .map_err(|e| format!("Failed to get absolute path: {e}"))?;

        let abs_path_str = abs_path.to_str().ok_or("Invalid absolute path")?;
        let server_url = format!("file://{abs_path_str}");

        if !new_content.contains("[testing]") {
            new_content.push_str("\n[testing]\n");
            new_content.push_str(&format!("Server = {}\n", server_url));
            new_content.push_str("SigLevel = Optional TrustAll\n");
        }
    } else {
        if let Some(pos) = new_content.find("[testing]") {
            new_content.truncate(pos);
            new_content = new_content.trim_end().to_string();
            new_content.push('\n');
        }
    }

    fs::write(&all_paths.build_pacman_config_file, new_content)
        .map_err(|e| format!("Failed to save pacman config: {e}"))?;

    Ok(())
}
