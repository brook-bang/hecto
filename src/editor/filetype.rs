use std::{default, fmt::{write, Display},};

#[derive(Default,PartialEq, Eq,Debug,Clone, Copy)]
pub enum FileType {
    Rust,
    #[default]
    Text,
}

impl Display for FileType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rust => write!(formatter, "Rust"),
            Self::Text => write!(formatter, "Text"),
        }
    }
}