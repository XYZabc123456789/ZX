//cli.rs
use clap::*;
use core::num::NonZero;
use is_terminal::*;
#[derive(Parser, Debug)]
#[command(name = "zxc", disable_version_flag = true,
    group(
        ArgGroup::new("input_source")
            .required(true)
            .args(["input", "config", "version"])
    ))]
pub struct Args {
    /// Input file
    pub input: Option<String>,
    /// Output file
    #[arg(short, long)]
    pub output: Option<String>,
    /// Number of threads
    #[arg(short, long, default_value_t = auto_parallel())]
    pub threads: NonZero<usize>,
    /// Print Version
    #[arg(short = 'V', long = "version")]
    pub version: bool,
    /// Input config
    #[arg(short = 'c',long = "with-config",num_args = 0..=1, conflicts_with = "input")]
    pub config: Option<Option<String>>,
}

pub fn auto_parallel() -> NonZero<usize> {
    std::thread::available_parallelism().unwrap_or_else(|_| NonZero::new(1_usize).unwrap())
}

pub fn print_version() {
    let link = "github.com/XYZabc123456789/ZX";

    let link = if std::io::stdout().is_terminal() {
        format!("\x1b]8;;https://{link}\x07{link}\x1b]8;;\x07")
    } else {
        link.to_string()
    };

    println!("ZXC v{}", env!("CARGO_PKG_VERSION"));
    println!("This is the ZX compiler for the ZX language.");
    println!("ZXC is free software, licensed under the GNU General Public License (GPLv3)");
    println!("You can find the source code @ {link}");
    println!();
    println!("Run `--help` to see available commands");
}
