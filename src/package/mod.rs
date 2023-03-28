use std::fmt::Display;

pub mod config;
pub(crate) mod line;
pub(crate) mod link;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Install,
    Remove,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let action = match self {
            Self::Install => "install",
            Self::Remove => "remove",
        };
        write!(f, "{}", action)
    }
}
