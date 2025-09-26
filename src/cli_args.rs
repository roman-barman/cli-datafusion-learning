use clap::Parser;

///Data Fusion CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    ///Input path
    #[arg(short, long)]
    input: String,
}

impl CliArgs {
    pub fn input_path(&self) -> &str {
        &self.input
    }
}

