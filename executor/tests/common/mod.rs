use executor::{ExecutionContext, ExecutionResult, Executor, Language};

pub fn execute(
    language: Language,
    code: String,
    inputs: Vec<String>,
    time_limit_ms: Option<u32>,
) -> ExecutionResult {
    let executor = Executor::default();
    let context = ExecutionContext::builder()
        .language(language)
        .code(code)
        .inputs(inputs)
        .time_limit_ms(time_limit_ms.unwrap_or(1_000))
        .build();

    executor
        .execute(context)
        .expect("Error during execution test")
}
