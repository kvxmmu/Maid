use {
    clap::{
        Parser,
        Subcommand,
    },
    std::path::PathBuf,
};

#[derive(Debug, Subcommand)]
pub enum CliSub {
    /// Enumerate aarch64 assembler commands
    Enumerate { file: PathBuf },
}

#[derive(Debug, Parser)]
pub struct CliArgs {
    /// Action to perform
    #[clap(subcommand)]
    pub sub: CliSub,
}

impl CliArgs {
    pub fn from_args() -> Self {
        Self::parse()
    }
}
