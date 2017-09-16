use std::error;

use clap;

use config;
use super::executer::Executer;

pub struct Command<'c> {
    config: &'c config::command::Config,
}

impl<'c> Command<'c> {
    pub fn from_args(config: &'c config::command::Config, _args: &'c clap::ArgMatches<'c>) -> Self {
        debug!("ParamsListCommand::from_args");

        Command { config: config }
    }

    pub fn new(config: &'c config::command::Config) -> Self {
        debug!("ParamsListCommand::new");

        Command { config: config }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        debug!("ParamsListCommand::run");
        if let Some(params_config) = self.config.params.as_ref() {

            let exec = Executer::from_config(params_config);
            try!(exec.run());
        }
        Ok(())
    }
}
