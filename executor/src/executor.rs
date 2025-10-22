use rand::Rng;
use std::{fs, path::Path, process::Command, time::Instant};

use crate::{ExecutionContext, ExecutionResult, ExecutionStatus};

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
        let inputs_path = path.join("inputs/");

        fs::create_dir_all(&inputs_path).unwrap();

        for (i, input) in context.inputs.iter().enumerate() {
            fs::write(inputs_path.join(format!("{i}.in")), input.as_bytes())?;
            fs::File::create(inputs_path.join(format!("{i}.out")))?;
        }

        fs::write(
            path.join(format!("code{}", context.language.extension())),
            context.code.as_bytes(),
        )?;
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
                "--memory=16m",
                "--memory-swap=16m",
                "--pids-limit=16",
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

        let mut entries: Vec<_> = fs::read_dir(&inputs_path)?
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().is_some_and(|e| e == "out"))
            .collect();

        entries.sort_unstable_by_key(|f| f.file_name());

        let outputs: Vec<_> = entries
            .into_iter()
            .filter_map(|e| fs::read_to_string(e.path()).ok())
            .collect();

        let error = fs::read_to_string(path.join("error.txt"))?;
        let time = fs::read_to_string(path.join("time.txt"))?;

        fs::remove_dir_all(path)?;

        Ok(ExecutionResult {
            status: ExecutionStatus::from(run.status.code().unwrap_or(-1)),
            outputs,
            error,
            time_ms: start.elapsed().as_millis() as u32,
            time_execution_ms: time.trim().parse().unwrap_or(0),
        })
    }
}
