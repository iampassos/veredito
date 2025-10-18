use clap::Parser;

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

    Ok(())
}
