//mc lib.rs
mod cli;
use cli::*;
mod config_handler;
use config_handler::*;

use clap::{CommandFactory, Parser, error::ErrorKind};
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn run() {
    let args = Args::parse();
    if args.version {
        print_version();
        return;
    }
    let cwd = env::current_dir().unwrap_or_else(|e| {
        Args::command()
            .error(
                ErrorKind::Io,
                format!("could not determine current working directory: {e}"),
            )
            .exit()
    });
    let config; 
    match (args.input, args.config) {
        (Some(i), None) => {
            config = format!(r#"{{
                "direct-input": "{}"  
            }}"#, i)
        },
        (None, Some(c)) => {
            let conf_path = match c {
                Some(p) => PathBuf::from(p),
                None => detect_config(&cwd),
            };
            config = read_file(&conf_path);
        }
        _ => unreachable!("Either input or onfig is meant to be passed."),
    }

    handle_config(config);
}

pub fn read_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| {
        Args::command()
        .error(
            ErrorKind::Io,
            format!("could not read '{}': {e}", path.display()),
        )
        .exit();
    })
}

fn detect_config(cwd: &Path) -> PathBuf {
    let path = cwd.join("ZX.json");

    if path.exists() {
        path
    } else {
        Args::command()
        .error(
            ErrorKind::InvalidValue,
            "error: could not find a config file"
        )
        .exit();
    }
}
