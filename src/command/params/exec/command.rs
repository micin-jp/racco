use std::error;

use clap;
use config;

use super::{Program, Arguments};
use super::executer::Executer;

pub struct Command<'c> {
    config: &'c config::command::Config,
    program: Program<'c>,
    args: Arguments<'c>,
}

impl<'c> Command<'c> {
    pub fn from_args(
        config: &'c config::command::Config,
        clap_args: &'c clap::ArgMatches<'c>,
    ) -> Self {
        debug!("ParamsExecCommand::from_args");

        let program = clap_args.value_of("PROGRAM").unwrap();
        let args = match clap_args.values_of("ARGS") {
            Some(args) => args.collect(),
            None => Vec::new(),
        };

        Command {
            config: config,
            program: program,
            args: args,
        }
    }

    pub fn new(
        config: &'c config::command::Config,
        program: &'c Program<'c>,
        args: &'c Arguments<'c>,
    ) -> Self {
        debug!("ParamsExecCommand::new");

        Command {
            config: config,
            program: program,
            args: args.to_owned(),
        }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        debug!("ParamsExecCommand::run");
        if let Some(params_config) = self.config.params.as_ref() {
            let exec = Executer::from_config(params_config);
            try!(exec.run(&self.program, &self.args));
        }
        Ok(())
    }
}
