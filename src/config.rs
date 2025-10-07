use crate::cli::Config;
use anyhow::Result;
use std::fs;

use serde::Deserialize;

/// Configuration structure for loading from TOML files.
/// All fields are optional to allow partial configuration.
#[derive(Deserialize, Debug)]
pub struct FileConfig 
{
    pub input_files: Option<Vec<String>>,
    pub output_file: Option<String>,
    pub extra_flags: Option<Vec<String>>,
    pub assembler: Option<String>,
    pub assembler_flags: Option<Vec<String>>,
    pub target: Option<String>,
    pub verbose: Option<bool>,
    pub dry_run: Option<bool>,
    pub clean: Option<bool>,
    pub watch: Option<bool>,
    pub color: Option<String>,
}

impl From<FileConfig> for Config 
{
    fn from(value: FileConfig) -> Self 
    {
        Config 
        {
            input_files: value.input_files.unwrap_or_default(),
            output_file: value.output_file.unwrap_or_default(),
            extra_flags: value.extra_flags.unwrap_or_default(),
            assembler: value.assembler.unwrap_or_else(|| "as".to_string()),
            assembler_flags: value.assembler_flags.unwrap_or_default(),
            target: value.target.unwrap_or_else(|| {
                if cfg!(target_os = "macos") 
                {
                    "arm64".to_string()
                } else {
                    "x86_64".to_string()
                }
            }),
            verbose: value.verbose.unwrap_or(false),
            dry_run: value.dry_run.unwrap_or(false),
            clean: value.clean.unwrap_or(false),
            config_file: None,
            watch: value.watch.unwrap_or(false),
            color: value.color.unwrap_or_else(|| "auto".to_string()),
            completions: None,
        }
    }
}

/// Load configuration from a TOML file.
/// 
/// # Arguments
/// * `path` - Path to the TOML configuration file
/// 
/// # Returns
/// A Config struct with values loaded from the file
pub fn load_config(path: &str) -> Result<Config> 
{
    let contents = fs::read_to_string(path)?;
    let file_config: FileConfig = toml::from_str(&contents)?;
    Ok(file_config.into())
}

/// Auto-detect configuration file in current directory.
/// Searches for .rasm.toml, rasm.toml, .rasm/config.toml in order.
/// 
/// # Returns
/// Path to the configuration file if found, None otherwise
pub fn auto_detect_config() -> Option<String> 
{
    let candidates = vec![
        ".rasm.toml",
        "rasm.toml",
        ".rasm/config.toml",
    ];
    
    for candidate in candidates 
    {
        if std::path::Path::new(candidate).exists() 
        {
            return Some(candidate.to_string());
        }
    }
    
    None
}
