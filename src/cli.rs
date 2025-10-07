use clap::Parser;

/// Configuration for the RASM assembler and linker.
/// Can be specified via command-line arguments or a TOML configuration file.
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config 
{
    /// Input assembly file(s) (supports glob patterns)
    #[arg(required = true)]
    pub input_files: Vec<String>,

    /// Output binary file
    #[arg(short = 'o')]
    pub output_file: String,

    /// Extra flags for the linker (passed as-is)
    #[arg(last = true)]
    pub extra_flags: Vec<String>,

    /// Assembler to use (default: "as")
    #[arg(long, default_value = "as")]
    pub assembler: String,

    /// Additional flags to pass to the assembler
    #[arg(long)]
    pub assembler_flags: Vec<String>,

    /// Target architecture (default: "arm64" on macOS, "x86_64" otherwise)
    #[arg(long, default_value_t = default_target())]
    pub target: String,

    /// Enable verbose logging
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: bool,

    /// Dry-run: print commands without executing
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub dry_run: bool,

    /// Clean generated files (object and binary)
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub clean: bool,

    /// Path to configuration file (TOML format)
    #[arg(long)]
    pub config_file: Option<String>,
}

/// Returns the default target architecture based on the current platform.
/// Returns "arm64" on macOS, "x86_64" on other platforms.
fn default_target() -> String 
{
    if cfg!(target_os = "macos") 
    {
        "arm64".to_string()
    } else {
        "x86_64".to_string()
    }
}

/// Parse command-line arguments and return a Config struct.
pub fn parse_args() -> Config 
{
    Config::parse()
}

impl Config {
    /// Merge another configuration into self. Values from `other` override self if set.
    pub fn merge(&mut self, other: Config) 
    {
        if !other.input_files.is_empty() 
        {
            self.input_files = other.input_files;
        }
        if !other.output_file.is_empty() 
        {
            self.output_file = other.output_file;
        }
        if !other.extra_flags.is_empty() 
        {
            self.extra_flags = other.extra_flags;
        }
        if other.assembler != "as" 
        {
            self.assembler = other.assembler;
        }
        if !other.assembler_flags.is_empty() 
        {
            self.assembler_flags = other.assembler_flags;
        }
        if other.target != default_target() 
        {
            self.target = other.target;
        }
        if other.verbose 
        {
            self.verbose = true;
        }
        if other.dry_run 
        {
            self.dry_run = true;
        }
        if other.clean 
        {
            self.clean = true;
        }
    }

    /// Validate the configuration and return an error if invalid.
    pub fn validate(&self) -> anyhow::Result<()> 
    {
        if self.input_files.is_empty() 
        {
            return Err(anyhow::anyhow!("No input files specified"));
        }
        
        if self.output_file.is_empty() 
        {
            return Err(anyhow::anyhow!("Output file must be specified"));
        }
        
        if self.assembler.is_empty() 
        {
            return Err(anyhow::anyhow!("Assembler command cannot be empty"));
        }
        
        // Validate that input files don't have suspicious paths
        for input in &self.input_files 
        {
            if input.contains("..") 
            {
                return Err(anyhow::anyhow!(
                    "Input file paths cannot contain '..': {}", 
                    input
                ));
            }
        }
        
        // Validate output path
        if self.output_file.contains("..") 
        {
            return Err(anyhow::anyhow!(
                "Output file path cannot contain '..': {}", 
                self.output_file
            ));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        Config {
            input_files: vec!["test.s".to_string()],
            output_file: "test.out".to_string(),
            extra_flags: vec![],
            assembler: "as".to_string(),
            assembler_flags: vec![],
            target: "x86_64".to_string(),
            verbose: false,
            dry_run: false,
            clean: false,
            config_file: None,
        }
    }

    #[test]
    fn test_validate_valid_config() {
        let config = create_test_config();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_input_files() {
        let mut config = create_test_config();
        config.input_files = vec![];
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_empty_output_file() {
        let mut config = create_test_config();
        config.output_file = String::new();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_path_traversal_input() {
        let mut config = create_test_config();
        config.input_files = vec!["../evil.s".to_string()];
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_path_traversal_output() {
        let mut config = create_test_config();
        config.output_file = "../evil.out".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_merge_configs() {
        let mut config1 = create_test_config();
        let mut config2 = create_test_config();
        
        config2.input_files = vec!["new.s".to_string()];
        config2.verbose = true;
        
        config1.merge(config2);
        
        assert_eq!(config1.input_files, vec!["new.s".to_string()]);
        assert!(config1.verbose);
    }

    #[test]
    fn test_default_target() {
        let target = default_target();
        #[cfg(target_os = "macos")]
        assert_eq!(target, "arm64");
        
        #[cfg(not(target_os = "macos"))]
        assert_eq!(target, "x86_64");
    }
}
