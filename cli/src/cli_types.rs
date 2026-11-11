/*
 * ZXC, The ZX language compiler
 * Copyright (C) 2026  XYZabc123456789
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use clap::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "zxc", version, about = "The ZX Language Compiler")]
pub struct Cli {
    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Debug, Subcommand)]
pub enum Mode {
    /// Print license information
    License,

    /// Direct entrypoint compilation mode
    Run(DirectRunArgs),

    /// File-based config compilation mode
    Config(ConfigFileArgs),
}

#[derive(Debug, Args)]
pub struct DirectRunArgs {
    /// Source entrypoint file (mandatory in this mode)
    pub entrypoint: PathBuf,

    /// Optional artifact output name
    #[arg(short = 'o', long)]
    pub output_name: Option<String>,
}

#[derive(Debug, Args)]
pub struct ConfigFileArgs {
    /// Path to config file (mandatory in this mode)
    #[arg(short = 'c', long)]
    pub config: PathBuf,

    /// Target profile within the config file (mandatory in this mode)
    #[arg(short = 'p', long)]
    pub profile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerProfileConfig {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerConfig {
    pub entrypoint: PathBuf,
    #[serde(rename = "output")]
    pub output_name: Option<String>,
    pub profiles: Vec<CompilerProfileConfig>,
}

impl Default for CompilerProfileConfig {
    fn default() -> Self {
        Self {
            name: "Default".into(),
        }
    }
}
