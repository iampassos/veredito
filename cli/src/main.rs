use std::{
    fs::{self},
    path::Path,
    process::Command,
};

enum SubmissionResult {
    CompilationFailed,
    ExecutionFailed,
    WrongAnswer,
    Accepted,
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 4
        || !args[1].ends_with(".c")
        || !args[2].ends_with(".txt")
        || !args[3].ends_with(".txt")
    {
        return;
    }

    let path = Path::new("submissions/0");
    let source = args[1].clone();
    let input = args[2].clone();
    let expected = args[3].clone();
    let output = path.join("output.txt");

    fs::create_dir_all(path).unwrap();
    fs::copy(&source, path.join("source.c")).unwrap();
    fs::copy(&input, path.join("input.txt")).unwrap();
    fs::copy(&expected, path.join("expected.txt")).unwrap();
    fs::File::create(&output).unwrap();

    let build = Command::new("docker")
        .args(["build", ".", "-t", "sandbox"])
        .output()
        .expect("Failed to build");

    if !build.status.success() {
        eprintln!("Build failed");
        return;
    }

    let run = Command::new("docker")
        .args([
            "run",
            "--rm",
            "--network",
            "none",
            "--cpus=1",
            "--memory=64m",
            "--pids-limit=64",
            "-v",
            "./submissions/0/:/submission/",
            "sandbox",
        ])
        .output()
        .expect("Failed to run");

    let status = run.status.code().unwrap_or(-1);

    if status == 0 {
        if fs::read(expected).unwrap() == fs::read(output).unwrap() {
            println!("Correct Answer!");
        } else {
            println!("Wrong Answer!");
        }
    } else {
        println!("An error has occurred");
    }

    fs::remove_dir_all(path).unwrap();
}
