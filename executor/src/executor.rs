use rand::Rng;
use std::{fs, path::Path, process::Command, time::Instant};

use crate::{
    context::ExecutionContext,
    result::{ExecutionResult, ExecutionStatus},
};

#[derive(Default)]
pub struct Executor {}

impl Executor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(
        &self,
        context: ExecutionContext,
    ) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        let id = rng.gen_range(0..1000000);

        let str = format!("submissions/{id}");
        let path = Path::new(&str);

        fs::create_dir_all(path).unwrap();
        fs::write(path.join("input.txt"), context.input.as_bytes())?;
        fs::write(
            path.join(format!("code{}", context.language.extension())),
            context.code.as_bytes(),
        )?;
        fs::File::create(path.join("output.txt"))?;
        fs::File::create(path.join("error.txt"))?;
        fs::File::create(path.join("time.txt"))?;

        let start = Instant::now();

        let run = Command::new("docker")
            .args([
                "run",
                "-e",
                &format!("TIME_LIMIT={:.3}s", context.time_limit_ms as f32 / 1000.0),
                "--rm",
                "--network",
                "none",
                "--read-only",
                "--cpus=1",
                "--memory=64m",
                "--memory-swap=64m",
                "--pids-limit=64",
                "--cap-drop=ALL",
                "--security-opt=no-new-privileges",
                "-v",
                &format!("./{str}/:/submission/"),
                &format!("sandbox-{}", context.language.to_string().to_lowercase()),
                "sh",
                "-c",
                "run.sh",
            ])
            .output()?;

        let output = fs::read_to_string(path.join("output.txt"))?;
        let error = fs::read_to_string(path.join("error.txt"))?;
        let time = fs::read_to_string(path.join("time.txt"))?;

        fs::remove_dir_all(path)?;

        Ok(ExecutionResult {
            status: ExecutionStatus::from(run.status.code().unwrap_or(-1)),
            output,
            error,
            time_ms: start.elapsed().as_millis() as u32,
            time_execution_ms: time.trim().parse()?,
        })
    }
}
