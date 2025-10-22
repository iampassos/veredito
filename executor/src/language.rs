use std::fmt::Display;

#[derive(Debug)]
pub enum Language {
    C,
    Python,
}

impl TryFrom<&str> for Language {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "c" => Ok(Self::C),
            "py" | "python" => Ok(Self::Python),
            _ => Err("The language isn't supported"),
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::C => write!(f, "C"),
            Self::Python => write!(f, "Python"),
        }
    }
}

impl Language {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::C => ".c",
            Self::Python => ".py",
        }
    }

    pub fn supported() -> &'static [Self] {
        &[Self::C, Self::Python]
    }
}
