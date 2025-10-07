use crate::cli::Config;
use anyhow::Result;
use log::{debug, info};
use std::process::Command;

/// Link object files into a final executable.
/// 
/// Uses platform-specific linkers:
/// - macOS: clang with system libraries and SDK
/// - Other: ld
/// 
/// # Arguments
/// * `object_files` - List of object file paths to link
/// * `config` - Configuration containing linker settings and output path
pub fn link(object_files: &[String], config: &Config) -> Result<()> 
{
    let mut cmd = if cfg!(target_os = "macos") 
    {
        let mut c = Command::new("clang");
        c.args(&["-lSystem"]);
        let sdk_path = get_sdk_path().unwrap_or_else(|| String::from("/"));
        c.arg(format!("-Wl,-syslibroot,{}", sdk_path));
        c.args(&["-e", "_start", "-arch", &config.target]);
        c
    } else {
        Command::new("ld")
    };

    // Append all object files.
    for obj in object_files 
    {
        cmd.arg(obj);
    }
    cmd.arg("-o")
       .arg(&config.output_file);
    cmd.args(&config.extra_flags);

    if config.verbose || config.dry_run 
    {
        info!("Linking into output: {}", config.output_file);
        debug!("Linker command: {:?}", cmd);
    }
    if config.dry_run 
    {
        return Ok(());
    }
    let output = cmd.output()?;
    if !output.status.success() 
    {
        return Err(anyhow::anyhow!(
            "Linker failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

/// Get the macOS SDK path using xcrun (macOS only).
/// Returns None if not on macOS or if xcrun fails.
#[cfg(target_os = "macos")]
fn get_sdk_path() -> Option<String> 
{
    let output = Command::new("xcrun")
        .args(&["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .ok()?;
    if output.status.success() 
    {
        let path = String::from_utf8_lossy(&output.stdout);
        Some(path.trim().to_string())
    } else {
        None
    }
}

/// Stub implementation for non-macOS platforms.
#[cfg(not(target_os = "macos"))]
fn get_sdk_path() -> Option<String> 
{
    None
}

