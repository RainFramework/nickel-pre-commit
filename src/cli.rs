use crate::{
    eval::EvalCommand, format::FormatCommand,
};

#[derive(clap::Parser, Debug)]
/// The interpreter of the Nickel language.
#[command(
author,
about,
long_about = None
)]
pub struct Options {
    #[command(flatten)]
    pub global: GlobalOptions,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Parser, Debug)]
pub struct GlobalOptions {
    /// Configure when to output messages in color
    #[arg(long, global = true, value_enum, default_value_t)]
    pub color: clap::ColorChoice,
}

/// Available subcommands.
#[derive(clap::Subcommand, Debug)]
pub enum Command {
    /// Evaluate a Nickel program and pretty-print the result.
    Eval(EvalCommand),

    /// Format Nickel files
    Format(FormatCommand),
}