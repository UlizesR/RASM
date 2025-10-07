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

    // Validate configuration
    config.validate()?;

    // Clean mode: remove generated object and binary files.
    if config.clean 
    {
        for input in &config.input_files 
        {
            let obj_file = format!("{}.o", input);
            match fs::remove_file(&obj_file) {
                Ok(_) => info!("Removed object file: {}", obj_file),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    debug!("Object file not found (already cleaned): {}", obj_file);
                }
                Err(e) => {
                    log::warn!("Failed to remove object file {}: {}", obj_file, e);
                }
            }
        }
        match fs::remove_file(&config.output_file) {
            Ok(_) => info!("Removed output binary: {}", config.output_file),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                debug!("Output file not found (already cleaned): {}", config.output_file);
            }
            Err(e) => {
                log::warn!("Failed to remove output file {}: {}", config.output_file, e);
            }
        }
        return Ok(());
    }

    // Expand glob patterns in input files
    let expanded_inputs = assembler::expand_globs(&config.input_files)?;
    info!("Processing {} file(s)", expanded_inputs.len());

    // Assemble each input file.
    let mut object_files = Vec::new();
    for input in &expanded_inputs 
    {
        let obj = assembler::assemble(input, &config)?;
        object_files.push(obj);
    }

    // Link all object files into the final executable.
    linker::link(&object_files, &config)?;

    Ok(())
}

