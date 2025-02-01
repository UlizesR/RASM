use crate::cli::Config;
use anyhow::Result;
use std::fs;

use serde::Deserialize;

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
}

impl Into<Config> for FileConfig 
{
    fn into(self) -> Config 
    {
        Config 
        {
            input_files: self.input_files.unwrap_or_default(),
            output_file: self.output_file.unwrap_or_default(),
            extra_flags: self.extra_flags.unwrap_or_default(),
            assembler: self.assembler.unwrap_or_else(|| "as".to_string()),
            assembler_flags: self.assembler_flags.unwrap_or_default(),
            target: self.target.unwrap_or_else(|| {
                if cfg!(target_os = "macos") 
                {
                    "arm64".to_string()
                } else {
                    "x86_64".to_string()
                }
            }),
            verbose: self.verbose.unwrap_or(false),
            dry_run: self.dry_run.unwrap_or(false),
            clean: self.clean.unwrap_or(false),
            config_file: None,
        }
    }
}

pub fn load_config(path: &str) -> Result<Config> 
{
    let contents = fs::read_to_string(path)?;
    let file_config: FileConfig = toml::from_str(&contents)?;
    Ok(file_config.into())
}
