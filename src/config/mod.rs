use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Config {
    #[allow(dead_code)]
    pub fn load() -> Result<Self, ConfigError> {
        let mut builder = config::Config::builder();

        // Try to load from TOML files (lower priority)
        if let Some(toml_path) = find_config_file() {
            builder = builder
                .add_source(config::File::with_name(toml_path.to_str().unwrap()).required(false));
        }

        // Environment variables take priority
        builder = builder.add_source(
            config::Environment::with_prefix("WISHLIST")
                .separator("_")
                .try_parsing(true),
        );

        let config = builder.build()?;
        config.try_deserialize().map_err(ConfigError::Deserialize)
    }
}

fn find_config_file() -> Option<PathBuf> {
    // First, check current working directory
    let cwd_config = Path::new("wishlist.toml");
    if cwd_config.exists() {
        return Some(cwd_config.to_path_buf());
    }

    // Then check XDG config directory (with defaults)
    if let Some(config_dir) = dirs::config_dir() {
        let xdg_config = config_dir.join("wishlist").join("wishlist.toml");
        if xdg_config.exists() {
            return Some(xdg_config);
        }
    }

    None
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ConfigError {
    Config(config::ConfigError),
    Deserialize(config::ConfigError),
}

impl From<config::ConfigError> for ConfigError {
    fn from(err: config::ConfigError) -> Self {
        ConfigError::Config(err)
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Config(e) => write!(f, "Config error: {}", e),
            ConfigError::Deserialize(e) => write!(f, "Deserialization error: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::Mutex;
    use tempfile::TempDir;

    // Serialize test execution to avoid environment variable conflicts
    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_load_from_toml() {
        let _guard = TEST_MUTEX.lock().unwrap();
        // Clear any environment variables that might interfere
        let env_backup = std::env::var("WISHLIST_DATABASE_URL").ok();
        std::env::remove_var("WISHLIST_DATABASE_URL");

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("wishlist.toml");

        fs::write(
            &config_path,
            r#"
[database]
url = "postgresql://localhost/testdb"
"#,
        )
        .unwrap();

        // Change to temp directory so find_config_file finds our test file
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let config = Config::load().unwrap();
        assert_eq!(config.database.url, "postgresql://localhost/testdb");

        std::env::set_current_dir(original_dir).unwrap();

        // Restore environment variable if it was set
        if let Some(val) = env_backup {
            std::env::set_var("WISHLIST_DATABASE_URL", val);
        }
    }

    #[test]
    fn test_load_from_environment() {
        let _guard = TEST_MUTEX.lock().unwrap();
        // Backup and clear any existing value
        let env_backup = std::env::var("WISHLIST_DATABASE_URL").ok();
        std::env::set_var("WISHLIST_DATABASE_URL", "postgresql://localhost/envdb");

        let config = Config::load().unwrap();
        assert_eq!(config.database.url, "postgresql://localhost/envdb");

        // Restore or remove
        if let Some(val) = env_backup {
            std::env::set_var("WISHLIST_DATABASE_URL", val);
        } else {
            std::env::remove_var("WISHLIST_DATABASE_URL");
        }
    }

    #[test]
    fn test_environment_overrides_toml() {
        let _guard = TEST_MUTEX.lock().unwrap();
        // Backup environment variable
        let env_backup = std::env::var("WISHLIST_DATABASE_URL").ok();

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("wishlist.toml");

        fs::write(
            &config_path,
            r#"
[database]
url = "postgresql://localhost/tomldb"
"#,
        )
        .unwrap();

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Set environment variable that should override TOML
        std::env::set_var("WISHLIST_DATABASE_URL", "postgresql://localhost/envdb");

        let config = Config::load().unwrap();
        // Environment should override TOML
        assert_eq!(config.database.url, "postgresql://localhost/envdb");

        std::env::set_current_dir(original_dir).unwrap();

        // Restore environment variable
        if let Some(val) = env_backup {
            std::env::set_var("WISHLIST_DATABASE_URL", val);
        } else {
            std::env::remove_var("WISHLIST_DATABASE_URL");
        }
    }
}
