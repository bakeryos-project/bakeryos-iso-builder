use std::{env, fs::create_dir_all, path::PathBuf, sync::OnceLock};

#[derive(Clone, Debug)]
pub struct AllPaths {
    pub current_working_dir: PathBuf,
    pub work_dir: PathBuf,
    pub out_dir: PathBuf,
    pub pacman_config_file: PathBuf,
    pub build_dir: PathBuf,
    pub build_log_dir: PathBuf,
    pub build_pacman_config_file: PathBuf,
    pub package_cache_dir: PathBuf,
    pub testing_repo_dir: PathBuf,
    pub testing_repo_db_file: PathBuf,
    pub testing_package_dir: PathBuf,
    pub skel_dir: PathBuf,
}

static PATHS_CACHE: OnceLock<AllPaths> = OnceLock::new();

pub fn get_current_working_dir() -> Result<PathBuf, String> {
    let path = env::current_dir().map_err(|e| e.to_string())?;
    Ok(path)
}

pub fn get_all_paths() -> Result<&'static AllPaths, String> {
    let paths = PATHS_CACHE.get_or_init(|| {
        let cwd = get_current_working_dir().expect("failed to get current working dir");

        AllPaths {
            work_dir: cwd.join("work"),
            out_dir: cwd.join("out"),
            pacman_config_file: cwd.join("pacman.conf"),
            build_dir: cwd.join("build"),
            build_log_dir: cwd.join("build").join("logs"),
            build_pacman_config_file: cwd.join("build").join("pacman.conf"),
            package_cache_dir: cwd.join("build").join("packages").join("cache"),
            testing_repo_dir: cwd.join("build").join("repo").join("testing"),
            testing_repo_db_file: cwd
                .join("build")
                .join("repo")
                .join("testing")
                .join("testing.db"),
            testing_package_dir: cwd.join("testing").join("packages"),
            skel_dir: cwd.join("airootfs/etc/skel"),
            current_working_dir: cwd,
        }
    });

    Ok(paths)
}

pub fn init_file_system() -> Result<(), String> {
    let all_paths = get_all_paths()?;

    create_dir_all(&all_paths.build_dir).map_err(|e| e.to_string())?;
    create_dir_all(&all_paths.build_log_dir).map_err(|e| e.to_string())?;
    create_dir_all(&all_paths.package_cache_dir).map_err(|e| e.to_string())?;
    create_dir_all(&all_paths.testing_package_dir).map_err(|e| e.to_string())?;

    Ok(())
}
