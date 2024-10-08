use log::{ debug, warn };
use std::path::{Path, PathBuf};

const DEFAULT_WAKEFILE: &str = r#"# Wakefile Options
:active_directory = "."
:entry_name = "main.lua"

# Use LocalScripts on the client
client :: LocalScript

# Example Recipe
server -> ServerScriptService
client -> StarterPlayerScripts"#;

pub fn check_config() -> PathBuf {
    debug!("Checking for existance of wakefile");

    let names = vec![".wakefile", ".wake", "wakefile"];
    let mut found = false;
    let mut path = Path::new("wakefile").to_path_buf();

    for name in names {
        debug!("Checking for possible wakefile at {}", name);
        if std::fs::metadata(name).is_ok() {
            debug!("Found wakefile at {}", name);
            found = true;
            path = Path::new(name).to_path_buf();
            break;
        }
    }

    if !found {
        warn!("No wakefile found, creating a new one...");
        std::fs::write("wakefile", DEFAULT_WAKEFILE).expect("Failed to create wakefile");
    }

    return path;
}