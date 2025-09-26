use clap::Parser;

mod cli_args;

#[tokio::main]
async fn main() {
    let args = cli_args::CliArgs::parse();
    println!("{}", args.input_path().display());
}
