use std::process::Command;

fn main() {
    let build = Command::new("docker")
        .args(["build", ".", "-t", "judge"])
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
            "-v",
            "./submissions/0/:/submission/",
            "judge",
        ])
        .output()
        .expect("Failed to run");

    let exit_code = run.status.code().unwrap_or(-1);
    println!("Exit code: {}", exit_code);
}
