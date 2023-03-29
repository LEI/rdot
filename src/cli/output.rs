use clap::ValueEnum;

#[derive(Clone, Copy, Default, Debug, ValueEnum)]
pub(crate) enum Format {
    /// Output in an easily parseable format
    Raw,

    /// Output in json format
    Json,

    // #[clap(skip)]
    #[default]
    None,
}
