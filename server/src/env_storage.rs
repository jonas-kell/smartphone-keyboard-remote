use dotenvy::{from_path, from_path_iter, Error as DotenvError};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Ensures the `.env` file exists in the executable's directory and returns its path.
fn ensure_env_file() -> io::Result<PathBuf> {
    let exe_dir = env::current_exe()?.parent().unwrap().to_path_buf();
    let env_path = exe_dir.join(".env");

    if !env_path.exists() {
        File::create(&env_path)?;
    }

    Ok(env_path)
}

/// Reads the existing `.env` file into a `HashMap`.
fn read_env_file(path: &Path) -> Result<HashMap<String, String>, DotenvError> {
    if !path.exists() {
        return Ok(HashMap::new());
    }

    let vars = from_path_iter(path)?
        .filter_map(|result| match result {
            Ok((k, v)) => Some((k.to_string(), v.to_string())),
            Err(_) => None,
        })
        .collect();

    Ok(vars)
}

/// Writes a `HashMap` of key-value pairs to the `.env` file.
fn write_env_file(path: &Path, vars: &HashMap<String, String>) -> io::Result<()> {
    let mut file = File::create(path)?;

    for (key, value) in vars {
        writeln!(file, "{}={}", key, value)?;
    }

    Ok(())
}

/// Updates or inserts a key-value pair in the `.env` file.
pub fn update_env_file(key: &str, value: &str) -> io::Result<()> {
    let env_path = ensure_env_file()?;
    let mut vars = read_env_file(&env_path).unwrap_or_default();

    // Update or insert the key-value pair
    vars.insert(key.to_string(), value.to_string());

    // Write back to the file
    write_env_file(&env_path, &vars)
}

/// Reads a value from the `.env` file using `dotenvy`.
pub fn read_from_env(key: &str) -> Option<String> {
    env::remove_var(key);

    let env_path = ensure_env_file().ok()?;

    if from_path(&env_path).is_err() {
        eprintln!("Failed to load .env file from {}", env_path.display());
        return None;
    }

    env::var(key).ok()
}
