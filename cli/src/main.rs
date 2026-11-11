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
const LICENSE: &str = include_str!("../../LICENSE");
mod cli_types;
use clap::Parser;
use cli_types::Mode::*;
use cli_types::*;
use std::path::PathBuf;

fn main() {
    let config = Cli::parse();

    match config.mode {
        License => {
            print!("{LICENSE}");
        }
        Run(a) => {
            banner();
            ignite_compilation(
                a.entrypoint,
                a.output_name,
                CompilerProfileConfig::default(),
            );
        }
        Config(c) => {
            banner();
            let raw_config = std::fs::read_to_string(c.config).expect("Config file does not exist");
            let rconfig = serde_json::from_str(&raw_config);
            let config: CompilerConfig = match rconfig {
                Ok(c) => c,
                Err(e) => {
                    panic!("{e}");
                }
            };
            let profiles: Vec<CompilerProfileConfig> = config
                .profiles
                .into_iter()
                .filter(|pconf| pconf.name == c.profile)
                .collect();
            match profiles.len() {
                0 => {
                    panic!("No such profile");
                }
                1 => {
                    let profile = profiles.into_iter().next().expect("profile exists");
                    ignite_compilation(config.entrypoint, config.output_name, profile);
                }
                _ => {
                    panic!("Multiple profiles have the same name");
                }
            }
        }
    }
}

fn banner() {
    println!("ZXC  Copyright (C) 2026  XYZabc123456789");
    println!("This program comes with ABSOLUTELY NO WARRANTY.");
    println!(
        "This is free software, and you are welcome to redistribute it under certain conditions."
    );
    println!("For full license and warranty details, type `zxc license`");
}

fn ignite_compilation(
    entrypoint: PathBuf,
    output_name: Option<String>,
    profile: CompilerProfileConfig,
) {
    println!("entrypoint: {}", entrypoint.as_os_str().display());
    println!("output_name: {}", output_name.unwrap_or_default());
    println!("profile: {:?}", profile);
}
