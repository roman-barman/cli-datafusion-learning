use clap::Parser;
use std::path::PathBuf;

///Data Fusion CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    ///Input path
    #[arg(short, long)]
    input: PathBuf,
}

impl CliArgs {
    pub fn input_path(&self) -> &PathBuf {
        &self.input
    }
}

