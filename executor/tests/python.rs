use executor::{ExecutionStatus, Language};

mod common;

#[test]
fn run_program_successfully() {
    let code = "print(int(input()) * int(input()))".into();

    let result = common::execute(
        Language::Python,
        code,
        vec!["10\n10".into(), "20\n20".into(), "30\n30".into()],
        None,
    );

    assert_eq!(result.status, ExecutionStatus::Success);
    assert_eq!(result.outputs, ["100\n", "400\n", "900\n"]);
}

#[test]
fn fails_on_one_test_case_fail() {
    let code = "if int(input()) == 10: raise Exception('')".into();

    let result = common::execute(
        Language::Python,
        code,
        vec!["5".into(), "10".into(), "20".into()],
        None,
    );

    assert_eq!(result.status, ExecutionStatus::RuntimeError);
}

#[test]
fn fails_on_syntax_error() {
    let code = "print(".into();
    let result = common::execute(Language::Python, code, vec![], None);
    assert_eq!(result.status, ExecutionStatus::RuntimeError);
}

#[test]
fn fails_on_runtime_error() {
    let code = "1/0".into();
    let result = common::execute(Language::Python, code, vec![], None);
    assert_eq!(result.status, ExecutionStatus::RuntimeError);
}

#[test]
fn exceeds_time_limit() {
    let code = "while(True):\n pass".into();
    let result = common::execute(Language::Python, code, vec![], Some(10));
    assert_eq!(result.status, ExecutionStatus::TimeLimitExceeded);
}

#[test]
fn exceeds_memory_limit() {
    let code = "a = ' ' * (100*1024*1024)".into();
    let result = common::execute(Language::Python, code, vec![], None);
    assert_eq!(result.status, ExecutionStatus::MemoryLimitExceeded);
}
