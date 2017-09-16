use std::error;

use clap;

use config;
use super::executer::Executer;

pub struct Command<'c> {
    config: &'c config::command::Config,
    name: &'c str,
    value: &'c str,
}

impl<'c> Command<'c> {
    pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
        debug!("ParamsPutCommand::from_args");
        Command {
            config: config,
            name: args.value_of("NAME").unwrap(),
            value: args.value_of("VALUE").unwrap(),
        }
    }

    pub fn new(config: &'c config::command::Config, name: &'c str, value: &'c str) -> Self {
        debug!("ParamsPutCommand::new");
        Command {
            config: config,
            name: name,
            value: value,
        }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        debug!("ParamsPutCommand::run");
        if let Some(params_config) = self.config.params.as_ref() {

            let exec = Executer::from_config(&params_config);

            try!(exec.run(self.name, self.value));
        }
        Ok(())
    }
}
