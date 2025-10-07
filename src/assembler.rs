use crate::cli::Config;
use anyhow::Result;
use glob::glob;
use log::{debug, info};
use std::process::Command;

/// Expand glob patterns in input files to concrete file paths.
pub fn expand_globs(inputs: &[String]) -> Result<Vec<String>> 
{
    let mut expanded = Vec::new();
    
    for input in inputs 
    {
        if input.contains('*') || input.contains('?') || input.contains('[') 
        {
            let mut matched = false;
            for entry in glob(input)? 
            {
                matched = true;
                let path = entry?;
                expanded.push(path.to_string_lossy().to_string());
            }
            if !matched 
            {
                return Err(anyhow::anyhow!("No files match pattern: {}", input));
            }
        } else {
            expanded.push(input.clone());
        }
    }
    
    Ok(expanded)
}

/// Assemble a single assembly source file into an object file.
/// 
/// # Arguments
/// * `input` - Path to the assembly source file (must not contain glob patterns)
/// * `config` - Configuration containing assembler settings
/// 
/// # Returns
/// Path to the generated object file
pub fn assemble(input: &str, config: &Config) -> Result<String> 
{
    // Validate that glob patterns are handled at a higher level
    // Individual assemble calls should receive concrete file paths
    if input.contains('*') 
    {
        return Err(anyhow::anyhow!(
            "Glob patterns should be expanded before calling assemble. Got: {}", 
            input
        ));
    }
    
    let input_path = input.to_string();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_globs_no_patterns() {
        let inputs = vec!["file1.s".to_string(), "file2.s".to_string()];
        let result = expand_globs(&inputs).unwrap();
        assert_eq!(result, inputs);
    }

    #[test]
    fn test_expand_globs_invalid_pattern() {
        let inputs = vec!["nonexistent*.s".to_string()];
        let result = expand_globs(&inputs);
        assert!(result.is_err());
    }

    #[test]
    fn test_assemble_rejects_glob_pattern() {
        let config = Config {
            input_files: vec![],
            output_file: "test.out".to_string(),
            extra_flags: vec![],
            assembler: "as".to_string(),
            assembler_flags: vec![],
            target: "x86_64".to_string(),
            verbose: false,
            dry_run: false,
            clean: false,
            config_file: None,
        };
        
        let result = assemble("*.s", &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Glob patterns"));
    }

    #[test]
    fn test_assemble_dry_run() {
        let config = Config {
            input_files: vec![],
            output_file: "test.out".to_string(),
            extra_flags: vec![],
            assembler: "as".to_string(),
            assembler_flags: vec![],
            target: "x86_64".to_string(),
            verbose: false,
            dry_run: true,
            clean: false,
            config_file: None,
        };
        
        let result = assemble("test.s", &config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test.s.o");
    }
}
