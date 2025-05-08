use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    /// File of the dish calulator.
    /// @defaul: ./plan.md
    #[arg(short, long)]
    pub plan: Option<PathBuf>,

    /// Root path under which all dishes can be found.
    /// @defaul: ./.
    #[arg(short, long)]
    pub dish_root: Option<PathBuf>,
}
