use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config 
]{
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

fn default_target() -> String 
{
    if cfg!(target_os = "macos") 
    {
        "arm64".to_string()
    } else {
        "x86_64".to_string()
    }
}

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
}

