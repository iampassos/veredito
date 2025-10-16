use rand::Rng;
use std::{fs, path::Path, process::Command, time::Instant};

#[derive(Debug)]
pub enum Language {
    C,
    Python,
}

impl TryFrom<&str> for Language {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "c" => Ok(Self::C),
            "py" | "python" => Ok(Self::Python),
            _ => Err("The language isn't supported"),
        }
    }
}

impl Language {
    pub fn extension(&self) -> &str {
        match self {
            Self::C => ".c",
            Self::Python => ".py",
        }
    }

    pub fn execution(&self) -> &str {
        match self {
            Self::C => {
                r#"gcc code.c -o binary 2> error.txt || exit 1; timeout $TIME_LIMIT sh -c './binary < input.txt > output.txt 2>> error.txt; CODE=$?; [ $CODE -eq 0 ] && exit 0 || [ $CODE -eq 137 ] && exit 137 || exit 2'"#
            }
            Self::Python => {
                r#"timeout $TIME_LIMIT sh -c 'python3 code.py < input.txt > output.txt 2> error.txt; CODE=$?; [ $CODE -eq 0 ] && exit 0 || [ $CODE -eq 1 ] && exit 2 || [ $CODE -eq 137 ] && exit 137 || exit $CODE'"#
            }
        }
    }

    pub fn image(&self) -> &str {
        match self {
            Self::C => "sandbox-c",
            Self::Python => "sandbox-py",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExecutionStatus {
    Success,
    CompilationFailed,
    RuntimeError,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    InternalError(i32),
    Unknown(i32),
}

impl From<i32> for ExecutionStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Success,
            1 => Self::CompilationFailed,
            2 => Self::RuntimeError,
            124 => Self::TimeLimitExceeded,
            137 => Self::MemoryLimitExceeded,
            125..=127 => Self::InternalError(value),
            _ => Self::Unknown(value),
        }
    }
}

#[derive(Debug)]
pub struct ExecutionContext {
    pub language: Language,
    pub code: String,
    pub input: String,
    pub time_limit_ms: Option<u32>,
}

#[derive(Debug)]
pub struct ExecutionResult {
    pub status: ExecutionStatus,
    pub output: String,
    pub error: String,
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
        fs::File::create(path.join("error.txt"))?;

        let start = Instant::now();

        let run = Command::new("docker")
            .args([
                "run",
                "-e",
                &format!(
                    "TIME_LIMIT={:.3}s",
                    context.time_limit_ms.unwrap_or(1000) as f32 / 1000.0
                ),
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
                context.language.image(),
                "sh",
                "-c",
                context.language.execution(),
            ])
            .output()?;

        let output = fs::read_to_string(path.join("output.txt"))?;
        let error = fs::read_to_string(path.join("error.txt"))?;

        fs::remove_dir_all(path)?;

        Ok(ExecutionResult {
            status: ExecutionStatus::from(run.status.code().unwrap_or(-1)),
            output,
            error,
            time_ms: start.elapsed().as_millis() as u32,
        })
    }
}
