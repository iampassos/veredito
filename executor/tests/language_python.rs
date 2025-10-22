use executor::{ExecutionStatus, Language};

mod common;

#[test]
fn runs_python_program() {
    let code =
        "n=int(input())\narr = [int(input()) for _ in range(n)]\nfor i in reversed(arr):\n print(i)"
            .into();
    let inputs = vec![
        "10\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10".into(),
        "5\n1\n2\n3\n4\n5\n".into(),
        "3\n1\n2\n3\n".into(),
    ];

    let result = common::execute(Language::Python, code, inputs, None);

    assert_eq!(result.status, ExecutionStatus::Success);
    assert_eq!(
        result.outputs.first().unwrap(),
        "10\n9\n8\n7\n6\n5\n4\n3\n2\n1\n"
    );
    assert_eq!(result.outputs.get(1).unwrap(), "5\n4\n3\n2\n1\n");
    assert_eq!(result.outputs.get(2).unwrap(), "3\n2\n1\n");
}

#[test]
fn fails_python_syntax() {
    let code = "print(".into();
    let result = common::execute(Language::Python, code, vec![], None);
    assert_eq!(result.status, ExecutionStatus::RuntimeError);
}

#[test]
fn fails_python_runtime() {
    let code = "1/0".into();
    let result = common::execute(Language::Python, code, vec![], None);
    assert_eq!(result.status, ExecutionStatus::RuntimeError);
}

#[test]
fn time_limit_exceeded_python() {
    let code = "while(True):\n pass".into();
    let result = common::execute(Language::Python, code, vec![], Some(10));
    assert_eq!(result.status, ExecutionStatus::TimeLimitExceeded);
}

#[test]
fn memory_limit_exceeded_python() {
    let code = "a = ' ' * (100*1024*1024)".into();
    let result = common::execute(Language::Python, code, vec![], None);
    assert_eq!(result.status, ExecutionStatus::MemoryLimitExceeded);
}
