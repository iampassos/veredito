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

    pub fn execution(&self) -> &'static str {
        match self {
            Self::C => {
                r#"gcc code.c -o binary 2> error.txt || exit 1; timeout $TIME_LIMIT sh -c './binary < input.txt > output.txt 2>> error.txt; CODE=$?; [ $CODE -eq 0 ] && exit 0 || [ $CODE -eq 137 ] && exit 137 || exit 2'"#
            }
            Self::Python => {
                r#"timeout $TIME_LIMIT sh -c 'python3 code.py < input.txt > output.txt 2> error.txt; CODE=$?; [ $CODE -eq 0 ] && exit 0 || [ $CODE -eq 1 ] && exit 2 || [ $CODE -eq 137 ] && exit 137 || exit $CODE'"#
            }
        }
    }

    pub fn docker_image(&self) -> &'static str {
        match self {
            Self::C => "sandbox-c",
            Self::Python => "sandbox-py",
        }
    }

    pub fn supported() -> &'static [Self] {
        &[Self::C, Self::Python]
    }
}
