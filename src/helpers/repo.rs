use crate::{
    helpers::exec::exec_command,
    utils::{fs::copy_dir_all, paths::get_all_paths},
};
use glob::glob;

pub fn sync_testing_repo() -> Result<(), String> {
    let all_paths = get_all_paths()?;
    copy_dir_all(&all_paths.testing_package_dir, &all_paths.testing_repo_dir)?;
    Ok(())
}

pub fn build_testing_repo() -> Result<(), String> {
    let all_paths = get_all_paths()?;

    let db_temp_file = all_paths.testing_repo_db_file.join(".tar.zst");
    let pattern_buf = all_paths.testing_repo_dir.join("*.pkg.tar.zst");

    let pattern_str = pattern_buf.to_string_lossy();
    let entries = glob(&pattern_str).expect("Failed to read glob pattern");

    let db_temp_str = db_temp_file.to_string_lossy();

    for entry in entries {
        match entry {
            Ok(path) => {
                let path_str = path.to_string_lossy();
                let args = vec![db_temp_str.as_ref(), path_str.as_ref()];
                exec_command("repo-add", args, &all_paths.current_working_dir)?;
            }
            Err(e) => eprintln!("Glob error: {:?}", e),
        }
    }

    Ok(())
}
