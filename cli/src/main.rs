use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{fs, path};

use executor::{context, executor::Executor, language::Language};

#[derive(Parser)]
#[command(about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        #[arg(short, long, value_parser = file_exists)]
        source: path::PathBuf,

        #[arg(short, long, value_parser = file_exists)]
        input: path::PathBuf,

        #[arg(short, long, value_parser = file_exists)]
        expected: Option<path::PathBuf>,
    },
    Judge {
        #[arg(short, long, value_parser = file_exists)]
        source: path::PathBuf,

        #[arg(short, long, value_parser = dir_exists)]
        tests: path::PathBuf,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Commands::Run {
        source,
        input,
        expected,
    } = args.command
    {
        let executor = Executor::new();
        let context = context::ExecutionContext::builder()
            .language(Language::try_from(source.extension().unwrap().to_str().unwrap()).unwrap())
            .code(fs::read_to_string(source)?)
            .input(fs::read_to_string(input)?)
            .build();
        let results = executor.execute(context).unwrap();

        println!("{results:#?}");

        if let Some(e) = expected {
            println!(
                "Correct Answer: {:#?}",
                results.output == fs::read_to_string(e).expect("Invalid expected txt path")
            );
        }
    }

    Ok(())
}

fn file_exists(s: &str) -> Result<path::PathBuf, String> {
    let p = path::PathBuf::from(s);
    if p.exists() {
        if p.is_dir() {
            return Err("Path is a directory".into());
        } else if p.extension().is_none() {
            return Err("File doesn't have an extension".into());
        }
        Ok(p)
    } else {
        Err("File doesn't exist".into())
    }
}

fn dir_exists(s: &str) -> Result<path::PathBuf, String> {
    let p = path::PathBuf::from(s);
    if p.exists() {
        if !p.is_dir() {
            return Err("Path is not a directory".into());
        }
        Ok(p)
    } else {
        Err("Directory doesn't exist".into())
    }
}
