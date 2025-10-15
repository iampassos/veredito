use rand::Rng;
use std::{fs, path::Path, process::Command, time::Instant};

#[derive(Debug)]
pub enum Language {
    C,
}

impl TryFrom<&str> for Language {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "c" => Ok(Self::C),
            _ => Err("The language isn't supported"),
        }
    }
}

impl Language {
    pub fn extension(&self) -> &str {
        match self {
            Self::C => ".c",
        }
    }

    pub fn execution(&self) -> &str {
        match self {
            Self::C => "(gcc code.c || exit 1) && (./a.out < input.txt > output.txt || exit 2)",
        }
    }

    pub fn image(&self) -> &str {
        match self {
            Self::C => "sandbox-c",
        }
    }
}

#[derive(Debug)]
pub enum ExecutionStatus {
    Success,
    CompilationFailed,
    RuntimeError,
    Unknown(i32),
}

impl From<i32> for ExecutionStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Success,
            1 => Self::CompilationFailed,
            2 => Self::RuntimeError,
            _ => Self::Unknown(value),
        }
    }
}

#[derive(Debug)]
pub struct ExecutionContext {
    pub language: Language,
    pub code: String,
    pub input: String,
}

#[derive(Debug)]
pub struct ExecutionResult {
    pub status: ExecutionStatus,
    pub output: String,
    pub time_ms: u32,
}

#[derive(Default)]
pub struct Executor {}

impl Executor {
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

        let start = Instant::now();

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
                &format!("./{str}/:/submission/"),
                context.language.image(),
                "bash",
                "-c",
                context.language.execution(),
            ])
            .output()?;

        let output = fs::read_to_string(path.join("output.txt"))?;

        fs::remove_dir_all(path)?;

        Ok(ExecutionResult {
            status: ExecutionStatus::from(run.status.code().unwrap_or(-1)),
            output,
            time_ms: start.elapsed().as_millis() as u32,
        })
    }
}
