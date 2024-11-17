use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Action {
    // TODO: List,
    Install,
    Remove,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let action = match self {
            // Self::List => "list",
            Self::Install => "install",
            Self::Remove => "remove",
        };
        write!(f, "{}", action)
    }
}
