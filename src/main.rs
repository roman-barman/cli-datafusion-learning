use clap::Parser;
use datafusion::functions_aggregate::expr_fn::min;
use datafusion::prelude::{col, CsvReadOptions, SessionContext};

mod cli_args;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    let args = cli_args::CliArgs::parse();
    let ctx = SessionContext::new();
    let df = ctx.read_csv(args.input_path(), CsvReadOptions::new()).await?;

    let df = df.filter(col("a").lt_eq(col("b")))?
        .aggregate(vec![col("a")], vec![min(col("b"))])?
        .limit(0, Some(100))?;
    df.show().await?;
    Ok(())
}
