#[derive(Clone, Default, PartialEq)]
#[non_exhaustive]
pub enum Mode {
    #[default]
    Auto,
    Dark,
    Light,
}

impl From<String> for Mode {
    fn from(input: String) -> Self {
        match input.as_str() {
            "dark" => Self::Dark,
            "light" => Self::Light,
            _ => Self::Auto,
        }
    }
}

impl From<&str> for Mode {
    fn from(input: &str) -> Self {
        match input {
            "dark" => Self::Dark,
            "light" => Self::Light,
            _ => Self::Auto,
        }
    }
}

impl Into<String> for Mode {
    fn into(self) -> String {
        match self {
            Mode::Auto => String::from("auto"),
            Mode::Dark => String::from("dark"),
            Mode::Light => String::from("light"),
        }
    }
}

impl Mode {
    pub fn to_string(self) -> String {
        match self {
            Mode::Auto => String::from("auto"),
            Mode::Dark => String::from("dark"),
            Mode::Light => String::from("light"),
        }
    }
}
