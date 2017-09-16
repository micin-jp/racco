use std::error;

use clap;
use config;

use super::executer::Executer;

pub struct Command<'c> {
    config: &'c config::command::Config,
    name: &'c str,
}

impl<'c> Command<'c> {
    pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
        trace!("command::schedule_task::put::Command::run");

        Command {
            config: config,
            name: args.value_of("NAME").unwrap(),
        }
    }

    pub fn new(config: &'c config::command::Config, name: &'c str) -> Self {
        trace!("command::schedule_task::put::Command::new");

        Command {
            config: config,
            name: name,
        }
    }

    pub fn run(&self) -> Result<(), Box<error::Error>> {
        trace!("command::schedule_task::put::Command::run");

        if let Some(schedule_config_group) = self.config.schedule_task.as_ref() {
            for schedule_config in schedule_config_group {
                if schedule_config.name != self.name {
                    continue;
                }

                let schedule_put_exec = Executer::from_config(&schedule_config);
                try!(schedule_put_exec.run());
            }
        }

        Ok(())
    }
}
