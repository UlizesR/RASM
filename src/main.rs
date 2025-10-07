mod assembler;
mod cli;
mod config;
mod linker;

use anyhow::Result;
use clap::CommandFactory;
use cli::parse_args;
use env_logger;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, info};
use notify_debouncer_full::{new_debouncer, notify::RecursiveMode};
use owo_colors::OwoColorize;
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

fn main() -> Result<()> 
{
    // Initialize logging (set RUST_LOG to control verbosity, e.g., RUST_LOG=info).
    env_logger::init();

    let mut config = parse_args();

    // Handle shell completions generation
    if let Some(shell) = config.completions {
        let mut cmd = cli::Config::command();
        clap_complete::generate(
            shell,
            &mut cmd,
            "rasm",
            &mut std::io::stdout(),
        );
        return Ok(());
    }

    // Setup colored output
    let use_colors = match config.color.as_str() {
        "always" => true,
        "never" => false,
        "auto" | _ => atty::is(atty::Stream::Stdout),
    };

    // Auto-detect configuration file if not specified
    if config.config_file.is_none() {
        if let Some(auto_config) = config::auto_detect_config() {
            if use_colors {
                println!("{} {}", "✓ Found config:".green(), auto_config.bright_blue());
            } else {
                println!("✓ Found config: {}", auto_config);
            }
            let file_config = config::load_config(&auto_config)?;
            config.merge(file_config);
            config.config_file = Some(auto_config);
        }
    } else if let Some(ref conf_path) = config.config_file {
        let file_config = config::load_config(conf_path)?;
        config.merge(file_config);
    }

    debug!("Effective configuration: {:?}", config);

    // Validate configuration
    config.validate()?;

    // Watch mode
    if config.watch {
        return run_watch_mode(&config, use_colors);
    }

    // Clean mode: remove generated object and binary files.
    if config.clean {
        clean_files(&config, use_colors)?;
        return Ok(());
    }

    // Build
    build_project(&config, use_colors)?;

    Ok(())
}

/// Clean generated files
fn clean_files(config: &cli::Config, use_colors: bool) -> Result<()> {
    for input in &config.input_files {
        let obj_file = format!("{}.o", input);
        match fs::remove_file(&obj_file) {
            Ok(_) => {
                if use_colors {
                    println!("{} {}", "✓ Removed:".green(), obj_file.bright_black());
                } else {
                    info!("Removed object file: {}", obj_file);
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                debug!("Object file not found (already cleaned): {}", obj_file);
            }
            Err(e) => {
                if use_colors {
                    eprintln!("{} Failed to remove {}: {}", "⚠".yellow(), obj_file, e);
                } else {
                    log::warn!("Failed to remove object file {}: {}", obj_file, e);
                }
            }
        }
    }
    match fs::remove_file(&config.output_file) {
        Ok(_) => {
            if use_colors {
                println!("{} {}", "✓ Removed:".green(), config.output_file.bright_black());
            } else {
                info!("Removed output binary: {}", config.output_file);
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            debug!("Output file not found (already cleaned): {}", config.output_file);
        }
        Err(e) => {
            if use_colors {
                eprintln!("{} Failed to remove {}: {}", "⚠".yellow(), config.output_file, e);
            } else {
                log::warn!("Failed to remove output file {}: {}", config.output_file, e);
            }
        }
    }
    Ok(())
}

/// Build the project
fn build_project(config: &cli::Config, use_colors: bool) -> Result<()> {
    // Expand glob patterns in input files
    let expanded_inputs = assembler::expand_globs(&config.input_files)?;
    
    if use_colors {
        println!(
            "{} {} file(s)",
            "→ Processing".bright_blue().bold(),
            expanded_inputs.len().to_string().bright_yellow()
        );
    } else {
        info!("Processing {} file(s)", expanded_inputs.len());
    }

    // Setup progress bar
    let pb = if use_colors && !config.dry_run {
        let bar = ProgressBar::new(expanded_inputs.len() as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        Some(bar)
    } else {
        None
    };

    // Assemble files in parallel
    let object_files: Result<Vec<String>> = expanded_inputs
        .par_iter()
        .map(|input| {
            let result = assembler::assemble(input, config);
            if let Some(ref bar) = pb {
                bar.inc(1);
                if let Ok(ref obj) = result {
                    bar.set_message(format!("✓ {}", obj));
                }
            }
            result
        })
        .collect();

    let object_files = object_files?;

    if let Some(ref bar) = pb {
        bar.finish_with_message("Assembly complete");
    }

    // Link all object files into the final executable.
    if use_colors {
        println!("{} {}", "→ Linking".bright_blue().bold(), config.output_file.bright_yellow());
    }
    
    linker::link(&object_files, config)?;

    if use_colors {
        println!(
            "{} Build complete: {}",
            "✓".green().bold(),
            config.output_file.bright_green().bold()
        );
    } else {
        info!("Build complete: {}", config.output_file);
    }

    Ok(())
}

/// Run in watch mode
fn run_watch_mode(config: &cli::Config, use_colors: bool) -> Result<()> {
    use notify_debouncer_full::DebounceEventResult;
    
    if use_colors {
        println!("{} {}", "👁".bright_yellow(), "Watching for changes...".bright_blue().bold());
    } else {
        println!("Watching for changes...");
    }

    let (tx, rx) = mpsc::channel();

    let mut debouncer = new_debouncer(
        Duration::from_secs(1),
        None,
        move |result: DebounceEventResult| {
            if let Ok(events) = result {
                for event in events {
                    if event.paths.iter().any(|p| {
                        p.extension().and_then(|s| s.to_str()) == Some("s") ||
                        p.extension().and_then(|s| s.to_str()) == Some("asm")
                    }) {
                        let _ = tx.send(());
                        break;
                    }
                }
            }
        },
    )?;

    // Watch current directory
    debouncer.watch(Path::new("."), RecursiveMode::Recursive)?;

    // Initial build
    if let Err(e) = build_project(config, use_colors) {
        if use_colors {
            eprintln!("{} {}", "✗".red().bold(), e.to_string().red());
        } else {
            eprintln!("Build failed: {}", e);
        }
    }

    // Wait for changes
    loop {
        if rx.recv().is_ok() {
            if use_colors {
                println!("\n{} Rebuilding...", "→".bright_blue().bold());
            } else {
                println!("\nRebuilding...");
            }

            if let Err(e) = build_project(config, use_colors) {
                if use_colors {
                    eprintln!("{} {}", "✗".red().bold(), e.to_string().red());
                } else {
                    eprintln!("Build failed: {}", e);
                }
            }
        }
    }
}

