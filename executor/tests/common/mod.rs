use executor::{ExecutionContext, ExecutionResult, Executor, Language};

pub fn execute(
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
