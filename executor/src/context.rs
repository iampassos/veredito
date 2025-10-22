use crate::Language;

#[derive(Debug)]
pub struct ExecutionContext {
    pub(crate) language: Language,
    pub(crate) code: String,
    pub(crate) inputs: Vec<String>,
    pub(crate) time_limit_ms: u32,
}

impl ExecutionContext {
    pub fn new(
        language: Language,
        code: String,
        inputs: Vec<String>,
        time_limit_ms: u32,
    ) -> ExecutionContext {
        Self {
            language,
            code,
            inputs,
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
    inputs: Vec<String>,
    time_limit_ms: u32,
}

impl Default for ExecutionContextBuilder {
    fn default() -> Self {
        Self {
            language: None,
            code: None,
            inputs: vec![],
            time_limit_ms: 1_000,
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

    pub fn add_input(mut self, input: String) -> Self {
        self.inputs.push(input);
        self
    }

    pub fn inputs(mut self, inputs: Vec<String>) -> Self {
        self.inputs = inputs;
        self
    }

    pub fn time_limit_ms(mut self, time_limit_ms: u32) -> Self {
        self.time_limit_ms = time_limit_ms;
        self
    }

    pub fn build(self) -> ExecutionContext {
        ExecutionContext {
            language: self.language.expect("Language is required"),
            code: self.code.expect("Code is required"),
            inputs: self.inputs,
            time_limit_ms: self.time_limit_ms,
        }
    }
}
