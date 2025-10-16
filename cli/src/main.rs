use clap::Parser;
use std::{
    fs::{self},
    path::Path,
};

use executor::{ExecutionContext, Executor, Language};

#[derive(Parser)]
#[command(about)]
struct Args {
    #[arg(short, long)]
    source: String,

    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    expected: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let path = Path::new(&args.source);

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or("The source code path must be valid!")?;
    let language = Language::try_from(ext)?;

    let executor = Executor::default();
    let results = executor.execute(ExecutionContext {
        language,
        code: fs::read_to_string(args.source).unwrap(),
        input: fs::read_to_string(args.input).unwrap(),
        time_limit_ms: None,
    });

    println!("{:#?}", results);

    Ok(())
}
