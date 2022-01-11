use std::error;

use clap;
use config;

use super::executer::Executer;

pub struct Command<'c> {
    config: &'c config::command::Config,
    name: Option<&'c str>,
    all: bool,
}

impl<'c> Command<'c> {
    pub fn from_args(config: &'c config::command::Config, args: &'c clap::ArgMatches<'c>) -> Self {
        trace!("command::schedule_task::put::Command::run");

        Command {
            config: config,
            name: args.value_of("NAME"),
            all: args.is_present("ALL"),
        }
    }

    pub fn new(config: &'c config::command::Config, name: Option<&'c str>, all: bool) -> Self {
        trace!("command::schedule_task::put::Command::new");

        Command {
            config: config,
            name: name,
            all: all,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn error::Error>> {
        trace!("command::schedule_task::put::Command::run");

        if let Some(schedule_config_group) = self.config.schedule_task.as_ref() {
            for schedule_config in schedule_config_group {
                let mut runnable: bool = false;
                if let Some(name) = self.name {
                    if name == schedule_config.name {
                        runnable = true;
                    }
                }
                if self.all {
                    runnable = true;
                }
                if !runnable {
                    continue;
                }

                let schedule_put_exec = Executer::from_config(&schedule_config);
                try!(schedule_put_exec.run());
            }
        }

        Ok(())
    }
}
