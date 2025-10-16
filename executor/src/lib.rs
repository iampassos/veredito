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
            Self::C => {
                r#"(gcc code.c -o binary 2> error.txt || exit 1) && timeout $TIME_LIMIT bash -c './binary < input.txt > output.txt 2>> error.txt; CODE=$?; [ $CODE -eq 0 ] && exit 0 || [ $CODE -eq 137 ] && exit 137 || exit 2'"#
            }
        }
    }

    pub fn image(&self) -> &str {
        match self {
            Self::C => "sandbox-c",
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
    InternalError,
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
            125..=127 => Self::InternalError,
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
                "bash",
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

#[cfg(test)]
mod tests {
    use super::*;

    fn execute(
        language: Language,
        code: String,
        input: String,
        time_limit_ms: Option<u32>,
    ) -> ExecutionResult {
        let executor = Executor::default();
        executor
            .execute(ExecutionContext {
                language,
                code,
                input,
                time_limit_ms,
            })
            .expect("Error during execution test")
    }

    #[test]
    fn runs_c_program() {
        let code = r#"#include <stdio.h>
        int main() { int arr[10]; for (int i = 0; i < 10; i++) scanf("%d", &arr[i]); for (int i = 0; i < 10; i++) printf("%d\n", arr[10 - i - 1]); return 0; }"#
        .into();
        let input = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10".into();
        let output = "10\n9\n8\n7\n6\n5\n4\n3\n2\n1\n";
        let result = execute(Language::C, code, input, None);
        assert_eq!(result.status, ExecutionStatus::Success);
        assert_eq!(result.output, output);
    }

    #[test]
    fn fails_c_compilation() {
        let code = r#"int main() { return }"#.into();
        let result = execute(Language::C, code, "".into(), None);
        assert_eq!(result.status, ExecutionStatus::CompilationFailed);
    }

    #[test]
    fn fails_c_runtime() {
        let code = r#"int main() { int a = 10 / 0; return 0; }"#.into();
        let result = execute(Language::C, code, "".into(), None);
        assert_eq!(result.status, ExecutionStatus::RuntimeError);
    }

    #[test]
    fn time_limit_exceeded_c() {
        let code = r#"int main() { while(1) ; }"#.into();
        let result = execute(Language::C, code, "".into(), Some(10));
        assert_eq!(result.status, ExecutionStatus::TimeLimitExceeded);
    }

    #[test]
    fn memory_limit_exceeded_c() {
        let code = r#"#include <stdlib.h>
        int main() { while(1) malloc(1024 * 1024); return 0; }"#
            .into();
        let result = execute(Language::C, code, "".into(), None);
        assert_eq!(result.status, ExecutionStatus::MemoryLimitExceeded);
    }
}
