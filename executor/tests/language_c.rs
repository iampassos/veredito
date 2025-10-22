use executor::{ExecutionStatus, Language};

mod common;

#[test]
fn runs_c_program() {
    let code = r#"#include <stdio.h>
        int main() { int arr[10]; for (int i = 0; i < 10; i++) scanf("%d", &arr[i]); for (int i = 0; i < 10; i++) printf("%d\n", arr[10 - i - 1]); return 0; }"#
        .into();
    let input = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10".into();
    let output = "10\n9\n8\n7\n6\n5\n4\n3\n2\n1\n";
    let result = common::execute(Language::C, code, input, None);
    assert_eq!(result.status, ExecutionStatus::Success);
    assert_eq!(result.output, output);
}

#[test]
fn fails_c_compilation() {
    let code = "int main() { return }".into();
    let result = common::execute(Language::C, code, "".into(), None);
    assert_eq!(result.status, ExecutionStatus::CompilationFailed);
}

#[test]
fn fails_c_runtime() {
    let code = "int main() { int a = 10 / 0; return 0; }".into();
    let result = common::execute(Language::C, code, "".into(), None);
    assert_eq!(result.status, ExecutionStatus::RuntimeError);
}

#[test]
fn time_limit_exceeded_c() {
    let code = "int main() { while(1) ; }".into();
    let result = common::execute(Language::C, code, "".into(), Some(10));
    assert_eq!(result.status, ExecutionStatus::TimeLimitExceeded);
}

#[test]
fn memory_limit_exceeded_c() {
    let code = "#include <stdlib.h>\nint main() { while(1) malloc(1024 * 1024); return 0; }".into();
    let result = common::execute(Language::C, code, "".into(), None);
    assert_eq!(result.status, ExecutionStatus::MemoryLimitExceeded);
}
