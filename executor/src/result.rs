use std::fmt::Display;

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

impl Display for ExecutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "Success"),
            Self::CompilationFailed => write!(f, "Compilation Failed"),
            Self::RuntimeError => write!(f, "Runtime Error"),
            Self::TimeLimitExceeded => write!(f, "Time Limit Exceeded"),
            Self::MemoryLimitExceeded => write!(f, "Memory Limit Exceeded"),
            Self::InternalError(num) => write!(f, "Internal Error ({num})"),
            Self::Unknown(num) => write!(f, "Unknown Error ({num})"),
        }
    }
}

impl ExecutionStatus {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success)
    }
}

#[derive(Debug)]
pub struct ExecutionResult {
    pub status: ExecutionStatus,
    pub output: String,
    pub error: String,
    pub time_ms: u32,
    pub time_execution_ms: u32,
}
