use crate::cli::Config;
use anyhow::Result;
use glob::glob;
use log::{debug, info};
use std::process::Command;

pub fn assemble(input: &str, config: &Config) -> Result<String> 
{
    // Expand glob patterns if necessary.
    let mut input_path = input.to_string();
    if input.contains('*') 
    {
        if let Some(entry) = glob(input)?.next() 
        {
            input_path = entry?.to_string_lossy().to_string();
        } else {
            return Err(anyhow::anyhow!("No files match pattern {}", input));
        }
    }

    let obj_file = format!("{}.o", input_path);
    let mut cmd = Command::new(&config.assembler);
    cmd.arg(&input_path)
       .arg("-o")
       .arg(&obj_file);
    for flag in &config.assembler_flags 
    {
        cmd.arg(flag);
    }
    if config.verbose || config.dry_run 
    {
        info!("Assembling {} -> {}", input_path, obj_file);
        debug!("Assembler command: {:?}", cmd);
    }
    if config.dry_run 
    {
        return Ok(obj_file);
    }
    let output = cmd.output()?;
    if !output.status.success() 
    {
        return Err(anyhow::anyhow!(
            "Assembler failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(obj_file)
}

