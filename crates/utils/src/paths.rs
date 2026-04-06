use bevy::prelude::Resource;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

/// OS-standard locations for persistent application data.
///
/// Uses the `directories` crate to resolve standard platform config and data directories:
/// - macOS: `~/Library/Application Support/com.masoniis.vantablock/`
/// - Windows: `%AppData%\Roaming\masoniis\vantablock\` (Config) and `%AppData%\Local\masoniis\vantablock\` (Data)
/// - Linux: `~/.config/vantablock/` and `~/.local/share/vantablock/`
#[derive(Resource, Clone, Debug)]
pub struct PersistentPaths {
    /// Standard location for game assets (textures, models, configs).
    pub assets_dir: PathBuf,
    /// Standard location for the configuration file.
    pub config_dir: PathBuf,
    /// Standard location for large data files (world saves).
    /// Uses local data directory to avoid network roaming issues on Windows.
    pub saves_dir: PathBuf,
    /// Standard location for transient data (mesh caches, processed textures).
    pub cache_dir: PathBuf,
    /// Standard location for logs and session state.
    pub logs_dir: PathBuf,
}

impl PersistentPaths {
    /// Resolves and creates the standard project directories for the current platform.
    ///
    /// This function will attempt to create all necessary subdirectories on disk.
    /// It panics if the OS fails to provide a standard directory structure.
    pub fn resolve() -> Self {
        let proj_dirs = ProjectDirs::from("com", "masoniis", "vantablock")
            .expect("OS failed to provide standard directories");

        let config_dir = proj_dirs.config_dir().to_path_buf();
        let saves_dir = proj_dirs.data_local_dir().join("saves");
        let cache_dir = proj_dirs.cache_dir().to_path_buf();
        let logs_dir = proj_dirs
            .state_dir()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| proj_dirs.data_dir().join("logs"));

        // resolve the assets directory based on the executable location
        let exe_path = std::env::current_exe().expect("Failed to get executable path");
        let exe_dir = exe_path
            .parent()
            .expect("Executable has no parent directory");

        // handle macOS .app bundle structure (binary is in Contents/MacOS, assets in Contents/Resources)
        let mut assets_dir = if cfg!(target_os = "macos") && exe_dir.ends_with("MacOS") {
            exe_dir.parent().unwrap().join("Resources").join("assets")
        } else {
            // Windows, Linux, and standard adjacent setups
            exe_dir.join("assets")
        };

        // fallback for development: if assets are not adjacent to the binary,
        if !assets_dir.exists() {
            let cwd_assets = PathBuf::from("assets");
            if cwd_assets.exists() {
                assets_dir = cwd_assets;
            }
        }

        // ensure all directories exist so systems can write to them immediately
        let dirs = [&config_dir, &saves_dir, &cache_dir, &logs_dir];
        for path in dirs {
            fs::create_dir_all(path).ok();
        }

        Self {
            assets_dir,
            config_dir,
            saves_dir,
            cache_dir,
            logs_dir,
        }
    }

    /// Resolves paths relative to the project root for development and benchmarking.
    ///
    /// This ensures we use the config and assets from the source tree rather than
    /// the OS-standard application data folders.
    pub fn resolve_dev() -> Self {
        // traverse up to the workspace/monorepo root (where assets lives)
        let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = crate_root
            .parent()
            .and_then(|p| p.parent())
            .unwrap_or(&crate_root);

        let dev_data = workspace_root.join("target").join("dev_data");
        let config_dir = dev_data.join("config");

        // ensure dummy data directories exist for the bench
        fs::create_dir_all(&config_dir).ok();
        fs::create_dir_all(dev_data.join("saves")).ok();
        fs::create_dir_all(dev_data.join("cache")).ok();
        fs::create_dir_all(dev_data.join("logs")).ok();

        Self {
            assets_dir: workspace_root.join("assets"),
            config_dir,
            saves_dir: dev_data.join("saves"),
            cache_dir: dev_data.join("cache"),
            logs_dir: dev_data.join("logs"),
        }
    }
}
