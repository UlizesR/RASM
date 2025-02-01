mod assembler;
mod cli;
mod config;
mod linker;

use anyhow::Result;
use cli::parse_args;
use env_logger;
use log::{debug, info};
use std::fs;

fn main() -> Result<()> 
{
    // Initialize logging (set RUST_LOG to control verbosity, e.g., RUST_LOG=info).
    env_logger::init();

    let mut config = parse_args();

    // Optionally load configuration from a file.
    if let Some(ref conf_path) = config.config_file 
    {
        let file_config = config::load_config(conf_path)?;
        config.merge(file_config);
    }

    debug!("Effective configuration: {:?}", config);

    // Clean mode: remove generated object and binary files.
    if config.clean 
    {
        for input in &config.input_files 
        {
            let obj_file = format!("{}.o", input);
            let _ = fs::remove_file(&obj_file);
            info!("Removed object file: {}", obj_file);
        }
        let _ = fs::remove_file(&config.output_file);
        info!("Removed output binary: {}", config.output_file);
        return Ok(());
    }

    // Assemble each input file.
    let mut object_files = Vec::new();
    for input in &config.input_files 
    {
        let obj = assembler::assemble(input, &config)?;
        object_files.push(obj);
    }

    // Link all object files into the final executable.
    linker::link(&object_files, &config)?;

    Ok(())
}

