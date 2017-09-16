use std::error;

use clap;
use config;

use super::executer::Executer;

#[allow(dead_code)]
pub struct Command<'c> {
    config: &'c config::command::Config,
    name: &'c str,
}

impl<'c> Command<'c> {
    pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
        debug!("ScheduleTaskDeleteCommand::from_args");

        Command {
            config: config,
            name: args.value_of("NAME").unwrap(),
        }
    }

    pub fn new(config: &'c config::command::Config, name: &'c str) -> Self {
        debug!("ScheduleTaskDeleteCommand::new");

        Command {
            config: config,
            name: name,
        }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        debug!("ScheduleTaskDeleteCommand::run");

        let schedule_del_exec = Executer::new();
        try!(schedule_del_exec.run(self.name));

        Ok(())
    }
}
