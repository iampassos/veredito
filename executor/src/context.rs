use crate::language::Language;

#[derive(Debug)]
pub struct ExecutionContext {
    pub(crate) language: Language,
    pub(crate) code: String,
    pub(crate) input: String,
    pub(crate) time_limit_ms: u32,
}

impl ExecutionContext {
    pub fn new(
        language: Language,
        code: String,
        input: String,
        time_limit_ms: u32,
    ) -> ExecutionContext {
        Self {
            language,
            code,
            input,
            time_limit_ms,
        }
    }

    pub fn builder() -> ExecutionContextBuilder {
        ExecutionContextBuilder::new()
    }
}

pub struct ExecutionContextBuilder {
    language: Option<Language>,
    code: Option<String>,
    input: Option<String>,
    time_limit_ms: Option<u32>,
}

impl Default for ExecutionContextBuilder {
    fn default() -> Self {
        Self {
            language: None,
            code: None,
            input: None,
            time_limit_ms: Some(1_000),
        }
    }
}

impl ExecutionContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    pub fn input(mut self, input: String) -> Self {
        self.input = Some(input);
        self
    }

    pub fn time_limit_ms(mut self, time_limit_ms: u32) -> Self {
        self.time_limit_ms = Some(time_limit_ms);
        self
    }

    pub fn build(self) -> ExecutionContext {
        ExecutionContext {
            language: self.language.expect("Language is required"),
            code: self.code.expect("Code is required"),
            input: self.input.expect("Input is required"),
            time_limit_ms: self.time_limit_ms.unwrap_or_default(),
        }
    }
}
