use std::error;

use clap;

use super::executer::Executer;
use config;

pub struct Command<'c> {
    config: &'c config::command::Config,
    name: &'c str,
}

impl<'c> Command<'c> {
    pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
        trace!("command::params::get::Command::from_args");

        let name = args.value_of("NAME").unwrap();

        Command {
            config: config,
            name: name,
        }
    }

    pub fn new(config: &'c config::command::Config, name: &'c str) -> Self {
        trace!("command::params::get::Command::new");

        Command {
            config: config,
            name: name,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn error::Error>> {
        trace!("command::params::get::Command::run");
        if let Some(params_config) = self.config.params.as_ref() {
            let exec = Executer::from_config(params_config);
            r#try!(exec.run(&self.name));
        }
        Ok(())
    }
}
