mod cli;
mod error;
mod eval;
mod input;
mod format;

use std::process::ExitCode;
use crate::cli::{Command, Options};

fn main() -> ExitCode {
    let opts = <Options as clap::Parser>::parse();

    let result = match opts.command {
        Command::Eval(eval) => eval.run(opts.global),
        Command::Format(format) => format.run(opts.global),
    };

    match result {
        Ok(()) | Err(error::Error::CustomizeInfoPrinted) => ExitCode::SUCCESS,
        Err(error) => {
            error.report();
            ExitCode::FAILURE
        }
    }
}