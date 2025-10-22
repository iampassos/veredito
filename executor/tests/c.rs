use executor::{ExecutionStatus, Language};

mod common;

#[test]
fn run_program_successfully() {
    let code = r#"#include <stdio.h>
            int main() { int n1, n2; scanf("%d %d", &n1, &n2); printf("%d\n", n1 * n2); return 0; }"#
        .into();

    let result = common::execute(
        Language::C,
        code,
        vec!["10 10".into(), "20 20".into(), "30 30".into()],
        None,
    );

    assert_eq!(result.status, ExecutionStatus::Success);
    assert_eq!(result.outputs, ["100\n", "400\n", "900\n"]);
}

#[test]
fn fails_on_one_test_case_fail() {
    let code = r#"#include <stdio.h>
            int main() { int n; scanf("%d", &n); if (n == 10) return 2; return 0; }"#
        .into();

    let result = common::execute(
        Language::C,
        code,
        vec!["5".into(), "10".into(), "20".into()],
        None,
    );

    assert_eq!(result.status, ExecutionStatus::RuntimeError);
}

#[test]
fn fails_on_compilation_error() {
    let code = "int main() { return }".into();
    let result = common::execute(Language::C, code, vec![], None);
    assert_eq!(result.status, ExecutionStatus::CompilationFailed);
}

#[test]
fn fails_on_runtime_error() {
    let code = "int main() { int a = 10 / 0; return 0; }".into();
    let result = common::execute(Language::C, code, vec![], None);
    assert_eq!(result.status, ExecutionStatus::RuntimeError);
}

#[test]
fn exceeds_time_limit() {
    let code = "int main() { while(1) ; }".into();
    let result = common::execute(Language::C, code, vec![], Some(10));
    assert_eq!(result.status, ExecutionStatus::TimeLimitExceeded);
}

#[test]
fn exceeds_memory_limit() {
    let code = "#include <stdlib.h>\nint main() { while(1) malloc(1024 * 1024); return 0; }".into();
    let result = common::execute(Language::C, code, vec![], None);
    assert_eq!(result.status, ExecutionStatus::MemoryLimitExceeded);
}
