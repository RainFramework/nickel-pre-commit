use crate::{
    cli::GlobalOptions,
    error::{CliResult, ResultErrorExt},
    input::{InputOptions, Prepare},
};

#[derive(clap::Parser, Debug)]
pub struct EvalCommand {
    #[command(flatten)]
    pub input: InputOptions,
}

impl EvalCommand {
    pub fn run(self, global: GlobalOptions) -> CliResult<()> {
        let mut program = self.input.prepare(&global)?;

        program
            .eval_full()
            .map(|t| println!("{t}"))
            .report_with_program(program)
    }
}